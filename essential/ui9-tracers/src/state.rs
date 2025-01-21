use serde::Serialize;
use ui9::flow::EventFlow;
use ui9::names::Fqn;
use ui9::tracer::Tracer;

pub trait Value: Serialize + PartialEq {}
impl<T> Value for T where Self: Serialize + PartialEq {}

#[derive(Serialize, Clone)]
pub struct StateValue<T: Value> {
    current_state: T,
}

impl<T: Value> EventFlow for StateValue<T> {
    fn class() -> &'static str {
        "ui9.element.state"
    }
}

pub struct State<T: Value> {
    tracer: Tracer<StateValue<T>>,
    value: StateValue<T>,
}

impl<T: Value> State<T> {
    pub fn new(fqn: Fqn, initial_state: T) -> Self {
        let value = StateValue {
            current_state: initial_state,
        };
        let tracer = Tracer::new(fqn, &value);
        Self { tracer, value }
    }

    pub fn set_state(&mut self, new_state: T) {
        if new_state != self.value.current_state {
            self.value.current_state = new_state;
            self.tracer.trace(&self.value);
        }
    }
}
