use crate::flow::{Flow, PackedAction, PackedEvent, PackedState};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, OnEvent};
use crb::core::mpsc;
use crb::core::UniqueId;
use crb::send::Recipient;
use crb::superagent::{ManageSubscription, Subscription};

pub struct Recorder<F: Flow> {
    state: F,
    actions_tx: mpsc::UnboundedSender<F::Action>,
    // TODO: Add listeners here
}

impl<F: Flow> Recorder<F> {
    pub fn new(state: F, actions_tx: mpsc::UnboundedSender<F::Action>) -> Self {
        Self { state, actions_tx }
    }
}

impl<F: Flow> Agent for Recorder<F> {
    type Context = AgentSession<Self>;
}

impl<F: Flow> Recorder<F> {
    fn distribute(&mut self, event: F::Event) {
        self.state.apply(event);
        // TODO: Distirbute events to subscribers...
    }
}

// TODO: Eliminate the wrapper
pub struct Update<F: Flow> {
    pub event: F::Event,
}

#[async_trait]
impl<F: Flow> OnEvent<Update<F>> for Recorder<F> {
    async fn handle(&mut self, update: Update<F>, _ctx: &mut Context<Self>) -> Result<()> {
        self.distribute(update.event);
        Ok(())
    }
}

pub struct EventFlow {
    recipient: Recipient<PackedEvent>,
    // TODO: Packed Events listener here
}

impl Subscription for EventFlow {
    type State = PackedState;
}

#[async_trait]
impl<F: Flow> ManageSubscription<EventFlow> for Recorder<F> {
    async fn subscribe(
        &mut self,
        sub_id: UniqueId<EventFlow>,
        _ctx: &mut Context<Self>,
    ) -> Result<PackedState> {
        // TODO: Add a subscriber
        self.state.pack_state()
    }

    async fn unsubscribe(
        &mut self,
        sub_id: UniqueId<EventFlow>,
        _ctx: &mut Context<Self>,
    ) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
impl<F: Flow> OnEvent<PackedAction> for Recorder<F> {
    async fn handle(&mut self, action: PackedAction, _ctx: &mut Context<Self>) -> Result<()> {
        let action = F::unpack_action(&action)?;
        let reaction = self.state.reaction(&action);
        if let Some(event) = reaction {
            self.distribute(event);
        }
        self.actions_tx.send(action)?;
        Ok(())
    }
}
