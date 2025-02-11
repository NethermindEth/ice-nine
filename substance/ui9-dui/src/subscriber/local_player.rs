use super::{Act, Player, PlayerState};
use crate::flow::{Flow, PackedEvent};
use crate::hub::Hub;
use crate::publisher::{EventFlow, RecorderLink};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next, OnEvent};
use crb::core::Slot;
use crb::superagent::Entry;

impl<F: Flow> Player<F> for LocalPlayer<F> {
    fn from_state(state: PlayerState<F>) -> Self {
        Self {
            state,
            recorder: Slot::empty(),
            entry: Slot::empty(),
        }
    }
}

pub struct LocalPlayer<F: Flow> {
    state: PlayerState<F>,
    recorder: Slot<RecorderLink>,
    entry: Slot<Entry<EventFlow>>,
}

impl<F: Flow> Agent for LocalPlayer<F> {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl<F: Flow> DoAsync<Initialize> for LocalPlayer<F> {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        // Subscribing to events stream
        let hub = Hub::link()?;
        let fqn = self.state.fqn.clone();
        let mut recorder = hub.server.discover(fqn).await?;
        let recipient = ctx.recipient();
        let state_entry = recorder.subscribe(recipient).await?;

        // Assign the initial state
        let unpacked_state = F::unpack_state(&state_entry.state)?;
        self.state.allocate_state(unpacked_state);

        // Store subscription handle and a link to forward actions
        self.recorder.fill(recorder)?;
        self.entry.fill(state_entry.entry)?;
        Ok(Next::events())
    }

    // TODO: Try restart later if failed
}

#[async_trait]
impl<F: Flow> OnEvent<PackedEvent> for LocalPlayer<F> {
    async fn handle(&mut self, event: PackedEvent, _ctx: &mut Context<Self>) -> Result<()> {
        let event = F::unpack_event(&event)?;
        self.state.apply_event(event);
        Ok(())
    }
}

// TODO: Turn that into an interaction
#[async_trait]
impl<F: Flow> OnEvent<Act<F>> for LocalPlayer<F> {
    async fn handle(&mut self, action: Act<F>, _ctx: &mut Context<Self>) -> Result<()> {
        let recorder = self.recorder.get_mut()?;
        let packed_action = F::pack_action(&action.action)?;
        recorder.act(packed_action).await?;
        Ok(())
    }
}
