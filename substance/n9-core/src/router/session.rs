use super::RouterLink;
use crb::agent::{Agent, AgentSession, Next};

pub struct ReasoningSession {
    router: RouterLink,
}

impl Agent for ReasoningSession {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::events()
    }
}
