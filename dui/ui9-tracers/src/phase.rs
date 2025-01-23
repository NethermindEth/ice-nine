use serde::{Deserialize, Serialize};
use ui9::names::Fqn;
use ui9_dui::{Flow, Publisher};

pub trait PhaseValue: ToString + PartialEq {}
impl<T> PhaseValue for T where Self: ToString + PartialEq {}

pub struct Phase<P> {
    publisher: Publisher<PhaseState>,
    phase: P,
}

impl<P: PhaseValue> Phase<P> {
    pub fn new(fqn: Fqn, phase: P) -> Self {
        let state = PhaseState {
            phase: phase.to_string(),
        };
        let publisher = Publisher::new(fqn, state);
        Self { publisher, phase }
    }

    pub fn set_phase(&mut self, new_phase: P) {
        if new_phase != self.phase {
            self.phase = new_phase;
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum PhaseEvent {
    SetPhase { phase: String },
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PhaseState {
    phase: String,
}

impl Flow for PhaseState {
    type Event = PhaseEvent;
    type Action = ();

    fn apply(&mut self, event: Self::Event) {
        match event {
            PhaseEvent::SetPhase { phase } => {
                self.phase = phase;
            }
        }
    }
}
