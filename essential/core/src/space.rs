use crb::agent::{Agent, AgentSession};

pub struct Space {}

impl Agent for Space {
    type Context = AgentSession<Self>;
}
