use crate::flow::{Flow, PackedEvent};
use crate::hub::Hub;
use crate::publisher::{EventFlow, RecorderLink};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, Duty, Next, OnEvent};
use crb::core::{watch, Slot};
use crb::superagent::Entry;
use ui9::names::Fqn;

pub enum Ported<F> {
    Loading,
    Loaded(F),
}

pub struct Player<F: Flow> {
    fqn: Fqn,
    link: Slot<RecorderLink>,
    entry: Slot<Entry<EventFlow>>,
    state: watch::Sender<Ported<F>>,
}

impl<F: Flow> Player<F> {
    pub fn new(fqn: Fqn, state: watch::Sender<Ported<F>>) -> Self {
        Self {
            fqn,
            link: Slot::empty(),
            entry: Slot::empty(),
            state,
        }
    }
}

impl<F: Flow> Agent for Player<F> {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl<F: Flow> Duty<Initialize> for Player<F> {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        // Subscribing to events stream
        let hub = Hub::link()?;
        let mut link = hub.server.discover(self.fqn.clone()).await?;
        let recipient = ctx.recipient();
        let state_entry = link.subscribe(recipient).await?;

        // Assign the initial state
        let unpacked_state = F::unpack_state(&state_entry.state)?;
        let state = Ported::Loaded(unpacked_state);
        self.state.send(state)?;

        // Store subscription handle and a link to forward actions
        self.link.fill(link)?;
        self.entry.fill(state_entry.entry)?;
        Ok(Next::events())
    }

    // TODO: Try restart later if failed
}

#[async_trait]
impl<F: Flow> OnEvent<PackedEvent> for Player<F> {
    async fn handle(&mut self, action: PackedEvent, _ctx: &mut Context<Self>) -> Result<()> {
        Ok(())
    }
}

pub struct Act<F: Flow> {
    pub action: F::Action,
}

#[async_trait]
impl<F: Flow> OnEvent<Act<F>> for Player<F> {
    async fn handle(&mut self, action: Act<F>, _ctx: &mut Context<Self>) -> Result<()> {
        // TODO: Forward action to a recorder (hub)
        Ok(())
    }
}
