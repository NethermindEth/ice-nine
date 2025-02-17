use super::drainer::{from_stream, MessageSink};
use super::protocol::{Ui9Message, Ui9Request, Ui9Response};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, ManagedContext, Next, OnEvent};
use crb::core::Slot;
use crb::superagent::{Entry, Supervisor, SupervisorSession};
use futures::SinkExt;
use libp2p::Stream;
use ui9_dui::flow::PackedEvent;
use ui9_dui::hub::Hub;
use ui9_dui::publisher::{EventFlow, RecorderLink};

pub struct RelayPlayer {
    // State 1
    stream: Slot<Stream>,

    // State 2
    writer: Slot<MessageSink>,

    // State 3
    entry: Slot<Entry<EventFlow>>,
    recorder: Slot<RecorderLink>,
}

impl RelayPlayer {
    pub fn new(stream: Stream) -> Self {
        Self {
            stream: Slot::filled(stream),
            writer: Slot::empty(),
            entry: Slot::empty(),
            recorder: Slot::empty(),
        }
    }
}

impl Supervisor for RelayPlayer {
    type BasedOn = AgentSession<Self>;
    type GroupBy = ();
}

impl Agent for RelayPlayer {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for RelayPlayer {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let stream = self.stream.take()?;
        let (drainer, writer) = from_stream(stream);
        ctx.assign(drainer, (), ());
        self.writer.fill(writer)?;
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<Result<Ui9Message>> for RelayPlayer {
    async fn handle(&mut self, msg: Result<Ui9Message>, ctx: &mut Context<Self>) -> Result<()> {
        log::trace!("Imcoming UI9 request: {msg:?}");
        match msg? {
            Ui9Message::Request(request) => {
                match request {
                    Ui9Request::Subscribe(fqn) => {
                        if self.entry.is_filled() {
                            return Err(anyhow!("Trying to subscribe twice"));
                        }
                        // Subscribing to events stream
                        let hub = Hub::link()?;
                        let mut recorder = hub.server.discover(fqn).await?;
                        let recipient = ctx.recipient();
                        let state_entry = recorder.subscribe(recipient).await?;
                        let state = state_entry.state;
                        self.send(state.into()).await?;
                        self.entry.fill(state_entry.entry)?;
                        self.recorder.fill(recorder)?;
                    }
                    Ui9Request::Action(action) => {
                        let recorder = self.recorder.get_mut()?;
                        recorder.act(action).await?;
                    }
                    Ui9Request::Unsubscribe => {
                        ctx.shutdown();
                    }
                }
                Ok(())
            }
            Ui9Message::Response(_response) => {
                Err(anyhow!("Response is not expected for relay stream"))
            }
        }
    }
}

impl RelayPlayer {
    async fn send(&mut self, response: Ui9Response) -> Result<()> {
        let writer = self.writer.get_mut()?;
        let message = Ui9Message::from(response);
        writer.send(message).await?;
        Ok(())
    }
}

#[async_trait]
impl OnEvent<PackedEvent> for RelayPlayer {
    async fn handle(&mut self, event: PackedEvent, _ctx: &mut Context<Self>) -> Result<()> {
        self.send(event.into()).await?;
        Ok(())
    }
}
