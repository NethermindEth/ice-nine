mod client;
mod listener;
mod local_player;
mod remote_player;
mod watcher;

pub use client::{HubClient, HubClientLink};
pub use listener::Listener;
pub use local_player::LocalPlayer;

use crate::flow::Flow;
use crb::core::{mpsc, watch};
use ui9::names::Fqn;

#[derive(Debug, Clone)]
pub enum Ported<F> {
    Loading,
    Loaded(F),
}

pub struct PlayerSetup<F: Flow> {
    pub fqn: Fqn,
    pub state: watch::Sender<Ported<F>>,
    /// An optional channel for sending all events
    pub events: Option<mpsc::UnboundedSender<F::Event>>,
}

pub struct Act<F: Flow> {
    pub action: F::Action,
}
