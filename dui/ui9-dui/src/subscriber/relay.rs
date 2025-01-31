use crate::flex::FlexCodec;
use crate::flow::PackedEvent;
use crate::hub::Hub;
use crate::protocol::{Message, Request, Response};
use crate::publisher::{EventFlow, RecorderLink};
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent};
use crb::core::Slot;
use crb::superagent::{Drainer, Entry, OnItem, Supervisor, SupervisorSession};
use futures::{AsyncReadExt, Sink, SinkExt, StreamExt};
use libp2p::Stream;
use std::pin::Pin;
use tokio::io;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tokio_util::compat::FuturesAsyncReadCompatExt;
use ui9::names::Fqn;

pub struct Relay {
    // State 1
    stream: Slot<Stream>,

    // State 2
    writer: Slot<Pin<Box<dyn Sink<Message, Error = Error> + Send>>>,

    // State 3
    fqn: Option<Fqn>,
    entry: Slot<Entry<EventFlow>>,
    recorder: Slot<RecorderLink>,
}

impl Relay {
    pub fn new(stream: Stream) -> Self {
        Self {
            stream: Slot::filled(stream),
            writer: Slot::empty(),
            fqn: None,
            entry: Slot::empty(),
            recorder: Slot::empty(),
        }
    }
}

impl Supervisor for Relay {
    type GroupBy = ();
}

impl Agent for Relay {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for Relay {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let stream = self.stream.take()?;
        let stream = stream.compat();
        let codec = FlexCodec::<Message>::new();
        let framed = Framed::new(stream, codec);
        let (writer, reader) = framed.split();
        let drainer = Drainer::new(reader);
        ctx.assign(drainer, (), ());
        self.writer.fill(Box::pin(writer))?;
        Ok(Next::events())
    }
}

#[async_trait]
impl OnItem<Result<Message>> for Relay {
    async fn on_item(
        &mut self,
        event: Result<Message>,
        _: (),
        ctx: &mut Context<Self>,
    ) -> Result<()> {
        let event = event?;
        match event {
            Message::Request(request) => {
                match request {
                    Request::Subscribe(fqn) => {
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
                    Request::Action(action) => {
                        let recorder = self.recorder.get_mut()?;
                        recorder.act(action)?;
                    }
                    Request::Unsubscribe => {
                        // TODO: Drop the stream
                    }
                }
                Ok(())
            }
            Message::Response(_response) => {
                Err(anyhow!("Response is not expected for relay stream"))
            }
        }
    }
}

impl Relay {
    async fn send(&mut self, response: Response) -> Result<()> {
        let writer = self.writer.get_mut()?;
        let message = Message::from(response);
        writer.send(message).await?;
        Ok(())
    }
}

#[async_trait]
impl OnEvent<PackedEvent> for Relay {
    async fn handle(&mut self, event: PackedEvent, _ctx: &mut Context<Self>) -> Result<()> {
        self.send(event.into()).await?;
        Ok(())
    }
}
