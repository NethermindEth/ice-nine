use super::recorder::{Recorder, Update};
use crate::flow::Flow;
use crate::hub::Hub;
use crb::agent::{Equip, RunAgent, StopAddress};
use crb::core::{mpsc, sync::Mutex};
use crb::runtime::InteractiveRuntime;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ui9::names::Fqn;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracerInfo {
    pub fqn: Fqn,
    // TODO: Use `Class` wrapper
    pub class: String,
}

pub struct Tracer<F: Flow> {
    recorder: StopAddress<Recorder<F>>,
    actions: mpsc::UnboundedReceiver<F::Action>,
}

impl<F: Flow> Tracer<F> {
    pub fn new(fqn: Fqn, state: F) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let recorder = Recorder::new(state, tx);
        let runtime = RunAgent::new(recorder);
        let address = runtime.address();
        if let Some(hub) = Hub::link() {
            let info = TracerInfo {
                fqn,
                class: F::class().into(),
            };
            hub.server.add_recorder(info, runtime).ok();
        }
        Self {
            recorder: address.equip(),
            actions: rx,
        }
    }

    pub fn event(&self, event: F::Event) {
        let update = Update { event };
        self.recorder.event(update).ok();
    }
}
