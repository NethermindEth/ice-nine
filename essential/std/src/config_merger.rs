use crb::agent::{Agent, AgentSession};

pub struct ConfigMerger {}

impl Agent for ConfigMerger {
    type Context = AgentSession<Self>;
    type Output = ();
}
