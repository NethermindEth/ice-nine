use crb::agent::{Agent, AgentSession};
use crb::superagent::{Supervisor, SupervisorSession};

pub struct Substance {}

impl Supervisor for Substance {
    type BasedOn = AgentSession<Self>;
    type GroupBy = ();
}

impl Agent for Substance {
    type Context = SupervisorSession<Self>;
}
