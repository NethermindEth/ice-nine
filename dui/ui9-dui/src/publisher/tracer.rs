use super::recorder::Recorder;
use crate::flow::Flow;
use crate::hub::Hub;
use crb::agent::{Address, Equip, RunAgent, StopAddress};
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

#[derive(Clone)]
pub struct Tracer<F: Flow> {
    recorder: Arc<StopAddress<Recorder<F>>>,
}

impl<F: Flow> Tracer<F> {
    pub fn new(fqn: Fqn, state: F) -> Self {
        let recorder = Recorder::new(state);
        let runtime = RunAgent::new(recorder);
        let address = runtime.address();
        if let Some(hub) = Hub::link() {
            let info = TracerInfo {
                fqn,
                class: F::class().into(),
            };
            hub.server.add_recorder(info, runtime).ok();
        }
        // TODO: Send the runtime to the HUB
        Self {
            recorder: Arc::new(address.equip()),
        }
    }

    pub fn event(&self, event: F::Event) {
        self.recorder.event(event).ok();
    }
}
