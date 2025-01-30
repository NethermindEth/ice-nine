use crb::agent::{Agent, AgentSession};
use ui9::names::Fqn;

pub struct Relay {
    fqn: Fqn,
}

impl Relay {
    pub fn new(fqn: Fqn) -> Self {
        Self { fqn }
    }
}

impl Agent for Relay {
    type Context = AgentSession<Self>;
}
