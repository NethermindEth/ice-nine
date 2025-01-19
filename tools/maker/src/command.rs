use crb::agent::{Agent, AgentSession};

pub struct CommandWatcher {
}

impl Agent for CommandWatcher {
    type Context = AgentSession<Self>;
}
