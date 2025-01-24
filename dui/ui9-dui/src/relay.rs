use crate::connector::Connector;
use crb::agent::{Agent, AgentSession, Address};

/// A hub server that keep information about remote components.
pub struct Relay {
    connector: Address<Connector>,
}

impl Relay {
    pub fn new(connector: Address<Connector>) -> Self {
        Self {
            connector,
        }
    }
}

impl Agent for Relay {
    type Context = AgentSession<Self>;
}
