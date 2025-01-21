use derive_more::Display;
use serde::{Deserialize, Serialize};
use ui9::names::Fqn;
use ui9_tracers::State;

#[derive(Display, PartialEq, Eq)]
pub enum ActorState {
    Created,
    Active,
    Done,
}

pub struct TracerPack {
    pub state: State<ActorState>,
}

impl TracerPack {
    pub fn root(name: &str) -> Self {
        let fqn = Fqn::root(name);
        Self {
            state: State::new(fqn, ActorState::Created),
        }
    }

    pub fn active(&mut self) {
        self.state.set_state(ActorState::Active);
    }

    pub fn done(&mut self) {
        self.state.set_state(ActorState::Done);
    }
}
