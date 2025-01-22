use serde::{Deserialize, Serialize};
use ui9::names::Fqn;
use ui9_dui::Tracer;
use ui9_flow::Flow;

#[derive(Serialize, Clone)]
pub struct ProgressValue {
    progress: u32,
}

pub struct Progress {
    tracer: Tracer<ProgressState>,
    current: u64,
    total: u64,
    value: ProgressValue,
    /// Adds extra digits after `.`
    precision: u64,
}

impl Progress {
    pub fn new(fqn: Fqn, total: u64) -> Self {
        let state = ProgressState { value: 0.0 };
        let tracer = Tracer::new(fqn, state);
        let value = ProgressValue { progress: 0 };
        Self {
            tracer,
            current: 0,
            total,
            value,
            precision: 100, // 100.00
        }
    }

    pub fn set_value(&mut self, value: u64) {
        self.current = value;
        let prev_value = self.value.progress;
        let new_value = (self.current * 100 * self.precision / self.total) as u32;
        if prev_value != new_value {
            self.value.progress = new_value;
            let event = ProgressEvent::SetProgress {
                value: self.value.progress as f32 / self.precision as f32,
            };
            self.tracer.event(event);
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ProgressEvent {
    SetProgress { value: f32 },
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProgressState {
    /// A progress value in the range: [0; 1]
    value: f32,
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
