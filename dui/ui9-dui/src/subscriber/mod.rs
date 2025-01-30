mod client;
mod listener;
mod local_player;
mod remote_player;
mod watcher;

pub use client::{HubClient, HubClientLink};
pub use listener::Listener;
pub use local_player::LocalPlayer;

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
        let state = P::default();
        let tracer = Listener::<P>::new(peer, fqn);
        Self {
            driver: P::Driver::from(tracer),
        }
    }

    pub fn unified() -> Self
    where
        P: Unified,
    {
        Self::new(None, P::fqn())
    }
}

#[derive(Debug, Clone)]
pub enum Ported<F> {
    Loading,
    Loaded(F),
}

impl<F> Ported<F> {
    pub fn loaded(&self) -> Option<&F> {
        if let Self::Loaded(state) = self {
            Some(state)
        } else {
            None
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

pub struct PlayerSetup<F: Flow> {
    pub fqn: Fqn,
    /// An optional channel for sending all events
    pub event_tx: mpsc::UnboundedSender<SubEvent<F>>,
}

impl<F: Flow> PlayerSetup<F> {
    // TODO: Methods assign state, etc
    // Don't create a watch channel manually

    pub fn send(&self, event: SubEvent<F>) {
        if !self.event_tx.is_closed() {
            // TODO: Logging
            self.event_tx.send(event).ok();
        }
    }
}

pub struct Act<F: Flow> {
    pub action: F::Action,
}

pub struct PortedState<F: Flow> {
    state_rx: watch::Receiver<Ported<F>>,
}

impl<F: Flow> Clone for PortedState<F> {
    fn clone(&self) -> Self {
        Self {
            state_rx: self.state_rx.clone(),
        }
    }
}

impl<F: Flow> PortedState<F> {
    fn new(state_rx: watch::Receiver<Ported<F>>) -> Self {
        Self { state_rx }
    }
}

impl<F: Flow> PortedState<F> {
    pub fn borrow(&self) -> watch::Ref<Ported<F>> {
        self.state_rx.borrow()
    }
}
