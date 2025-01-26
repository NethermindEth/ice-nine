mod client;
mod listener;
mod local_player;
mod remote_player;

pub use client::{HubClient, HubClientLink};
pub use listener::Listener;
pub use local_player::LocalPlayer;

#[derive(Debug, Clone)]
pub enum Ported<F> {
    Loading,
    Loaded(F),
    Stale(F),
}

#[derive(Debug, Clone)]
pub enum TelePorted<F> {
    Loading,
    Loaded(F),
    Stale(F),
}
