use super::{Act, PlayerSetup, Ported, State, SubEvent};
use crate::flow::{Flow, PackedEvent};
use crate::hub::Hub;
use crate::publisher::{EventFlow, RecorderLink};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, Duty, Next, OnEvent};
use crb::core::{watch, Slot};
use crb::superagent::Entry;

pub struct LocalPlayer<F: Flow> {
    setup: PlayerSetup<F>,
    recorder: Slot<RecorderLink>,
    entry: Slot<Entry<EventFlow>>,
    // TODO: Consider to move to PlayerSetup
    state_tx: Option<watch::Sender<F>>,
}

impl<F: Flow> LocalPlayer<F> {
    pub fn new(setup: PlayerSetup<F>) -> Self {
        Self {
            setup,
            recorder: Slot::empty(),
            entry: Slot::empty(),
            state_tx: None,
        }
    }
}

impl<F: Flow> Agent for LocalPlayer<F> {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl<F: Flow> Duty<Initialize> for LocalPlayer<F> {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        // Subscribing to events stream
        let hub = Hub::link()?;
        let fqn = self.setup.fqn.clone();
        let mut recorder = hub.server.discover(fqn).await?;
        let recipient = ctx.recipient();
        let state_entry = recorder.subscribe(recipient).await?;

        // Assign the initial state
        let unpacked_state = F::unpack_state(&state_entry.state)?;
        let (state_tx, state_rx) = watch::channel(unpacked_state);
        let state = State::new(state_rx);
        let event = SubEvent::State(state);
        self.setup.send(event);

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
        if let Some(state_tx) = self.state_tx.as_mut() {
            state_tx.send_modify(|state| {
                state.apply(event.clone());
            });
            let event = SubEvent::Event(event);
            self.setup.send(event);
        }
        Ok(())
    }
}

#[async_trait]
impl<F: Flow> OnEvent<Act<F>> for LocalPlayer<F> {
    async fn handle(&mut self, action: Act<F>, _ctx: &mut Context<Self>) -> Result<()> {
        let recorder = self.recorder.get_mut()?;
        let packed_action = F::pack_action(&action.action)?;
        recorder.act(packed_action)?;
        Ok(())
    }
}
