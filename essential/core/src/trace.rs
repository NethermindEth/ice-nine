use derive_more::Display;
use ui9::names::Fqn;
use ui9_tracers::Phase;

#[derive(Display, PartialEq, Eq)]
pub enum ActorPhase {
    Created,
    Active,
    Done,
}

pub struct TracerPack {
    pub state: Phase<ActorPhase>,
}

impl TracerPack {
    pub fn root(name: &str) -> Self {
        let fqn = Fqn::root(name);
        Self {
            state: Phase::new(fqn, ActorPhase::Created),
        }
    }

    pub fn active(&mut self) {
        self.state.set_phase(ActorPhase::Active);
    }

    pub fn done(&mut self) {
        self.state.set_phase(ActorPhase::Done);
    }
}
