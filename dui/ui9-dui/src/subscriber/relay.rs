use crate::flex::FlexCodec;
use crate::flow::PackedEvent;
use crate::hub::Hub;
use crate::protocol::Message;
use crate::publisher::EventFlow;
use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Agent, Context, Duty, Next, OnEvent};
use crb::core::Slot;
use crb::superagent::{Drainer, Entry, OnItem, Supervisor, SupervisorSession};
use futures::{AsyncReadExt, Sink, StreamExt};
use libp2p::Stream;
use tokio::io;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tokio_util::compat::FuturesAsyncReadCompatExt;
use ui9::names::Fqn;

pub struct Relay {
    fqn: Option<Fqn>,
    entry: Slot<Entry<EventFlow>>,
    stream: Slot<Stream>,
    writer: Slot<Box<dyn Sink<Message, Error = Error> + Send>>,
}

impl Relay {
    pub fn new(stream: Stream) -> Self {
        Self {
            fqn: None,
            entry: Slot::empty(),
            stream: Slot::filled(stream),
            writer: Slot::empty(),
        }
    }
}

impl Supervisor for Relay {
    type GroupBy = ();
}

impl Agent for Relay {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl Duty<Initialize> for Relay {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let stream = self.stream.take()?;
        let stream = stream.compat();
        let codec = FlexCodec::<Message>::new();
        let framed = Framed::new(stream, codec);
        let (writer, reader) = framed.split();
        let drainer = Drainer::new(reader);
        ctx.assign(drainer, (), ());
        self.writer.fill(Box::new(writer))?;
        Ok(Next::events())
    }
}

#[async_trait]
impl OnItem<Result<Message>> for Relay {
    async fn on_item(
        &mut self,
        event: Result<Message>,
        _: (),
        _ctx: &mut Context<Self>,
    ) -> Result<()> {
        let event = event?;
        Ok(())
    }
}

/*
struct Initialize;

#[async_trait]
impl Duty<Initialize> for Relay {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        // Subscribing to events stream
        let hub = Hub::link()?;
        let fqn = self.fqn.clone();
        let mut recorder = hub.server.discover(fqn).await?;
        let recipient = ctx.recipient();
        let state_entry = recorder.subscribe(recipient).await?;

        // TODO: Forward the state

        self.entry.fill(state_entry.entry)?;
        Ok(Next::events())
    }

    // TODO: Try restart later if failed
}

#[async_trait]
impl OnEvent<PackedEvent> for Relay {
    async fn handle(&mut self, event: PackedEvent, _ctx: &mut Context<Self>) -> Result<()> {
        // TODO: Forward the event
        Ok(())
    }
}
*/
