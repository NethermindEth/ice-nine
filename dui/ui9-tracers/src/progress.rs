use serde::{Deserialize, Serialize};
use ui9::flow::EventFlow;
use ui9::names::Fqn;
use ui9::tracer::Tracer;
use ui9_flow::Flow;

#[derive(Serialize, Clone)]
pub struct ProgressValue {
    progress: u32,
}

impl EventFlow for ProgressValue {
    fn class() -> &'static str {
        "ui9.progress"
    }
}

pub struct Progress {
    tracer: Tracer<ProgressValue>,
    current: u64,
    total: u64,
    value: ProgressValue,
    // TODO: Add precision - `100` default
}

impl Progress {
    pub fn new(fqn: Fqn, total: u64) -> Self {
        let tracer = Tracer::new(fqn);
        let value = ProgressValue { progress: 0 };
        tracer.trace(&value);
        Self {
            tracer,
            current: 0,
            total,
            value,
        }
    }

    pub fn set_value(&mut self, value: u64) {
        self.current = value;
        let prev_value = self.value.progress;
        let new_value = (self.current * 100 / self.total) as u32;
        if prev_value != new_value {
            self.value.progress = new_value;
            self.tracer.trace(&self.value);
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ProgressEvent {
    SetProgress { value: f64 },
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProgressState {
    /// A progress value in the range: [0; 1]
    value: f64,
}

impl Flow for ProgressState {
    type Event = ProgressEvent;
    type Action = ();

    fn apply(&mut self, event: Self::Event) {
        match event {
            ProgressEvent::SetProgress { value } => {
                self.value = value;
            }
        }
    }
}
