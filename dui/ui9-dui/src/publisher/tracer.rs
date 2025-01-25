use super::recorder::{Recorder, Update};
use super::server::HubServer;
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
    // TODO: Use `Class` wrapper
    pub class: String,
}

pub struct Tracer<F: Flow> {
    recorder: StopAddress<Recorder<F>>,
    actions: Option<mpsc::UnboundedReceiver<F::Action>>,
}

impl<F: Flow> Tracer<F> {
    pub fn new(fqn: Fqn, state: F) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let recorder = Recorder::new(state, tx);
        let runtime = RunAgent::new(recorder);
        let address = runtime.address();
        let info = TracerInfo {
            class: F::class().into(),
        };
        HubServer::add_recorder(fqn, info, runtime);
        Self {
            recorder: address.equip(),
            actions: Some(rx),
        }
    }

    pub fn ignore_actions(&mut self) {
        self.actions.take();
    }

    pub fn event(&self, event: F::Event) {
        let update = Update { event };
        self.recorder.event(update).ok();
    }
}
