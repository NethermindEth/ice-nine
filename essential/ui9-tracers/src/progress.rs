use serde::Serialize;
use ui9::flow::EventFlow;
use ui9::names::Fqn;
use ui9::tracer::Tracer;

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
}

impl Progress {
    pub fn new(fqn: Fqn, total: u64) -> Self {
        let value = ProgressValue { progress: 0 };
        let tracer = Tracer::new(fqn, &value);
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

pub struct ProgressDc {
    value: u32,
}
