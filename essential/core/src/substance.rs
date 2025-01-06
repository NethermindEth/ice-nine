use crb::agent::{Agent, Standalone, Supervisor, SupervisorSession};

pub struct Substance {}

impl Standalone for Substance {}

impl Substance {
    pub fn new() -> Self {
        Self {}
    }
}

impl Agent for Substance {
    type Context = SupervisorSession<Self>;
    type Output = ();
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Group {
    Molecules,
}

impl Supervisor for Substance {
    type GroupBy = Group;
}
