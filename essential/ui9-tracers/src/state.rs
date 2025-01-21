use serde::Serialize;
use ui9::flow::EventFlow;
use ui9::names::Fqn;
use ui9::tracer::Tracer;

pub trait Value: ToString + PartialEq {}
impl<T> Value for T where Self: ToString + PartialEq {}

#[derive(Serialize, Clone)]
pub struct StateValue<T: Value = String> {
    current_state: T,
}

impl EventFlow for StateValue {
    fn class() -> &'static str {
        "ui9.state"
    }
}

impl<T: Value> StateValue<T> {
    fn squash(&self) -> StateValue {
        StateValue {
            current_state: self.current_state.to_string(),
        }
    }
}

pub struct State<T: Value> {
    tracer: Tracer<StateValue>,
    value: StateValue<T>,
}

impl<T: Value> State<T> {
    pub fn new(fqn: Fqn, initial_state: T) -> Self {
        let value = StateValue {
            current_state: initial_state,
        };
        let tracer = Tracer::new(fqn, &value.squash());
        Self { tracer, value }
    }

    pub fn set_state(&mut self, new_state: T) {
        if new_state != self.value.current_state {
            self.value.current_state = new_state;
            self.tracer.trace(&self.value.squash());
        }
    }
}
