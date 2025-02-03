use crb::agent::Agent;
use crb::superagent::{Supervisor, SupervisorSession};

pub struct Substance {}

impl Supervisor for Substance {
    type GroupBy = ();
}

impl Agent for Substance {
    type Context = SupervisorSession<Self>;
}
