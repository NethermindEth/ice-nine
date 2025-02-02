use super::recorder::{Recorder, Update};
use super::server::HubServer;
use super::RecorderState;
use crate::flow::Flow;
use crate::subscriber::{drainer, Act};
use anyhow::{anyhow, Result};
use crb::agent::StopAddress;
use crb::core::mpsc;
use crb::superagent::Drainer;
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
    action_rx: Option<mpsc::UnboundedReceiver<Act<F>>>,
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

    pub fn receiver(&mut self) -> Result<mpsc::UnboundedReceiver<Act<F>>> {
        self.action_rx
            .take()
            .ok_or_else(|| anyhow!("Actions stream (drainer) has taken already."))
    }

    pub fn actions(&mut self) -> Result<Drainer<Act<F>>> {
        self.receiver().map(drainer::from_mpsc)
    }

    pub fn event(&self, event: F::Event) {
        let update = Update { event };
        self.recorder.event(update).ok();
    }
}
