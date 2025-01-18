use crb::agent::{Address, Agent, AgentSession};
use derive_more::From;

#[derive(Clone, From)]
pub struct SpaceLink {
    address: Address<Space>,
}

pub struct Space {}

impl Agent for Space {
    type Context = AgentSession<Self>;
}

impl Space {
    pub fn new() -> Self {
        Self {}
    }
}
