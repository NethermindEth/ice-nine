use super::recorder::{Recorder, Update};
use super::server::HubServer;
use super::RecorderState;
use crate::flow::Flow;
use crb::agent::StopAddress;
use crb::core::mpsc;
use serde::{Deserialize, Serialize};
use ui9::names::Fqn;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracerInfo {
    // TODO: Use `Class` wrapper
    pub class: String,
}

pub struct Tracer<F: Flow> {
    // TODO: Consider using StopRecipient
    recorder: StopAddress<Recorder<F>>,
    action_rx: Option<mpsc::UnboundedReceiver<F::Action>>,
}

impl<F: Flow> Tracer<F> {
    pub fn new(fqn: Fqn, state: F) -> Self {
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        let state = RecorderState { state, action_tx };
        let recorder = HubServer::spawn_recorder(fqn, state);
        Self {
            recorder,
            action_rx: Some(action_rx),
        }
    }

    pub fn ignore_actions(&mut self) {
        self.action_rx.take();
    }

    pub fn event(&self, event: F::Event) {
        let update = Update { event };
        self.recorder.event(update).ok();
    }
}
