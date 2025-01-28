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

pub struct PlayerSetup<F: Flow> {
    pub fqn: Fqn,
    pub state_tx: watch::Sender<Ported<F>>,
    /// An optional channel for sending all events
    pub event_tx: mpsc::UnboundedSender<F::Event>,
}

pub struct Act<F: Flow> {
    pub action: F::Action,
}

pub struct State<F: Flow> {
    state_rx: watch::Receiver<Ported<F>>,
}

impl<F: Flow> Clone for State<F> {
    fn clone(&self) -> Self {
        Self {
            state_rx: self.state_rx.clone(),
        }
    }
}

impl<F: Flow> State<F> {
    fn new(state_rx: watch::Receiver<Ported<F>>) -> Self {
        Self { state_rx }
    }
}

impl<F: Flow> State<F> {
    pub fn borrow(&self) -> watch::Ref<Ported<F>> {
        self.state_rx.borrow()
    }
}
