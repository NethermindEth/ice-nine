use crate::flow::PackedEvent;
use crate::hub::Hub;
use crate::publisher::EventFlow;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, Duty, Next, OnEvent};
use crb::core::Slot;
use crb::superagent::Entry;
use libp2p::Stream;
use ui9::names::Fqn;

pub struct Relay {
    fqn: Fqn,
    entry: Slot<Entry<EventFlow>>,
    stream: Stream,
}

impl Relay {
    pub fn new(fqn: Fqn, stream: Stream) -> Self {
        Self {
            fqn,
            entry: Slot::empty(),
            stream,
        }
    }
}

impl Agent for Relay {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }
}

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
