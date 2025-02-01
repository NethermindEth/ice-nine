mod client;
mod listener;
mod local_player;
mod relay_player;
mod remote_player;

pub use client::{HubClient, HubClientLink};
pub use listener::Listener;
pub use local_player::LocalPlayer;
pub use relay_player::RelayPlayer;

use crate::flow::{Flow, Unified};
use crb::core::{mpsc, watch};
use derive_more::{Deref, DerefMut};
use libp2p::PeerId;
use ui9::names::Fqn;

pub trait Subscriber: Flow + Default {
    type Driver: From<Listener<Self>>;
}

#[derive(Deref, DerefMut)]
pub struct Sub<P: Subscriber> {
    driver: P::Driver,
}

impl<P: Subscriber> Sub<P> {
    pub fn new(peer: Option<PeerId>, fqn: Fqn) -> Self {
        let tracer = Listener::<P>::new(peer, fqn);
        Self {
            driver: P::Driver::from(tracer),
        }
    }

    pub fn unified(peer: Option<PeerId>) -> Self
    where
        P: Unified,
    {
        Self::new(peer, P::fqn())
    }
}

#[derive(Debug)]
pub enum SubEvent<F: Flow> {
    State(State<F>),
    Event(F::Event),
    Lost,
}

#[derive(Debug)]
pub struct State<F: Flow> {
    state_rx: watch::Receiver<F>,
}

impl<F: Flow> State<F> {
    fn new(state_rx: watch::Receiver<F>) -> Self {
        Self { state_rx }
    }
}

impl<F: Flow> State<F> {
    pub fn borrow(&self) -> watch::Ref<F> {
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
        let (state_tx, state_rx) = watch::channel(new_state);
        self.state_tx = Some(state_tx);
        let state = State::new(state_rx);
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
