mod client;
pub mod drainer;
mod listener;
mod local_player;

pub use client::HubClient;
pub use listener::Listener;
pub use local_player::LocalPlayer;

use crate::flow::{Flow, Unified};
use crb::agent::{Agent, OnEvent};
use crb::core::{mpsc, watch};
use derive_more::{Deref, DerefMut, From};
use ui9::names::Fqn;

pub trait Subscriber: Flow + Default {
    type Driver: From<Listener<Self>> + Send;
}

#[derive(Deref, DerefMut)]
pub struct Sub<P: Subscriber> {
    driver: P::Driver,
}

impl<P: Subscriber> Sub<P> {
    pub fn local(fqn: Fqn) -> Self {
        let listener = Listener::<P>::local(fqn);
        Self::new(listener)
    }

    pub fn local_unified() -> Self
    where
        P: Unified,
    {
        Self::local(P::fqn())
    }

    pub fn new(listener: Listener<P>) -> Self {
        Self {
            driver: P::Driver::from(listener),
        }
    }
}

#[derive(Debug)]
pub enum SubEvent<F: Flow> {
    State(State<F>),
    Event(F::Event),
    Lost,
}

#[derive(Debug)]
pub struct State<T> {
    state_rx: watch::Receiver<T>,
}

impl<T> State<T> {
    pub fn new(state: T) -> (Self, watch::Sender<T>) {
        let (state_tx, state_rx) = watch::channel(state);
        (Self { state_rx }, state_tx)
    }
}

impl<T> State<T> {
    pub fn borrow(&self) -> watch::Ref<T> {
        self.state_rx.borrow()
    }
}

pub struct PlayerState<F: Flow> {
    pub fqn: Fqn,
    state_tx: Option<watch::Sender<F>>,
    /// An optional channel for sending all events
    event_tx: mpsc::UnboundedSender<SubEvent<F>>,
}

impl<F: Flow> PlayerState<F> {
    pub fn allocate_state(&mut self, new_state: F) {
        let (state, state_tx) = State::new(new_state);
        self.state_tx = Some(state_tx);
        let event = SubEvent::State(state);
        self.send(event);
    }

    pub fn apply_event(&mut self, event: F::Event) {
        if let Some(state_tx) = self.state_tx.as_mut() {
            state_tx.send_modify(|state| {
                state.apply(event.clone());
            });
            self.send(SubEvent::Event(event));
        }
    }

    pub fn deallocate_state(&mut self) {
        self.state_tx.take();
        self.send(SubEvent::Lost);
    }

    fn send(&self, event: SubEvent<F>) {
        if !self.event_tx.is_closed() {
            // TODO: Logging
            self.event_tx.send(event).ok();
        }
    }
}

pub struct Act<F: Flow> {
    pub action: F::Action,
}

pub trait Player<F: Flow>: Agent<Context: Default> + OnEvent<Act<F>> {
    type Args;

    fn from_state(args: Self::Args, state: PlayerState<F>) -> Self;
}
