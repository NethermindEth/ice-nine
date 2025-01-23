use crate::flow::Flow;
use crb::agent::{Agent, AgentSession};
use crb::core::watch;

pub enum Ported<F> {
    Loading,
    Loaded(F),
}

pub struct Player<F: Flow> {
    state: watch::Sender<Ported<F>>,
}

impl<F: Flow> Agent for Player<F> {
    type Context = AgentSession<Self>;
}
