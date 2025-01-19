use serde::{Deserialize, Serialize};
use uiio::fqn::Fqn;
use uiio_element::State;

#[derive(Deserialize, Serialize, PartialEq, Eq)]
pub enum ActorState {
    Created,
    Active,
    Done,
}

pub struct TracerPack {
    pub state: State<ActorState>,
}

impl TracerPack {
    pub fn new(fqn: Fqn) -> Self {
        Self {
            state: State::new(fqn, ActorState::Created),
        }
    }
}
