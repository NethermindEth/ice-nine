use super::RecorderState;
use crate::flow::{Flow, PackedAction, PackedEvent, PackedState};
use crate::subscriber::Act;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, AgentSession, Context, EventExt, OnEvent, UniAddress};
use crb::core::Unique;
use crb::send::{Recipient, Sender};
use crb::superagent::{ManageSubscription, StateEntry, SubscribeExt, Subscription};
use std::collections::HashSet;

#[derive(Clone)]
pub struct RecorderLink {
    address: UniAddress<dyn UniRecorder>,
}

impl RecorderLink {
    pub fn new(address: impl UniRecorder) -> Self {
        Self {
            address: UniAddress::new(address),
        }
    }

    pub async fn subscribe(
        &mut self,
        recipient: Recipient<PackedEvent>,
    ) -> Result<StateEntry<EventFlow>> {
        let msg = EventFlow { recipient };
        let state = self.address.subscribe(msg).await?;
        Ok(state)
    }

    pub fn act(&mut self, action: PackedAction) -> Result<()> {
        self.address.event(action)
    }
}

pub trait UniRecorder
where
    Self: Sync + Send + 'static,
    Self: SubscribeExt<EventFlow>,
    Self: EventExt<PackedAction>,
{
}

impl<F: Flow> UniRecorder for Address<Recorder<F>> {}

pub struct Recorder<F: Flow> {
    state: RecorderState<F>,
    subscribers: HashSet<Unique<EventFlow>>,
}

impl<F: Flow> Recorder<F> {
    pub fn new(state: RecorderState<F>) -> Self {
        Self {
            state,
            subscribers: HashSet::new(),
        }
    }
}

impl<F: Flow> Agent for Recorder<F> {
    type Context = AgentSession<Self>;
}

impl<F: Flow> Recorder<F> {
    fn distribute(&mut self, event: F::Event) -> Result<()> {
        let packed_event = F::pack_event(&event)?;
        self.state.apply(event);
        for subscriber in &self.subscribers {
            subscriber.recipient.send(packed_event.clone()).ok();
        }
        Ok(())
    }
}

// TODO: Eliminate the wrapper when `!Flow` restriction will be available for `F::Event`
pub struct Update<F: Flow> {
    pub event: F::Event,
}

#[async_trait]
impl<F: Flow> OnEvent<Update<F>> for Recorder<F> {
    async fn handle(&mut self, update: Update<F>, _ctx: &mut Context<Self>) -> Result<()> {
        self.distribute(update.event)?;
        Ok(())
    }
}

pub struct EventFlow {
    recipient: Recipient<PackedEvent>,
}

impl Subscription for EventFlow {
    type State = PackedState;
}

#[async_trait]
impl<F: Flow> ManageSubscription<EventFlow> for Recorder<F> {
    async fn subscribe(
        &mut self,
        sub: Unique<EventFlow>,
        _ctx: &mut Context<Self>,
    ) -> Result<PackedState> {
        let packed_state = self.state.pack_state()?;
        self.subscribers.insert(sub);
        Ok(packed_state)
    }

    async fn unsubscribe(
        &mut self,
        sub: Unique<EventFlow>,
        _ctx: &mut Context<Self>,
    ) -> Result<()> {
        self.subscribers.remove(&sub);
        Ok(())
    }
}

#[async_trait]
impl<F: Flow> OnEvent<PackedAction> for Recorder<F> {
    async fn handle(&mut self, action: PackedAction, _ctx: &mut Context<Self>) -> Result<()> {
        let action = F::unpack_action(&action)?;
        let reaction = self.state.reaction(&action);
        if let Some(event) = reaction {
            self.distribute(event)?;
        }
        let msg = Act { action };
        self.state.action_tx.send(msg)?;
        Ok(())
    }
}
