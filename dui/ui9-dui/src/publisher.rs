use crate::flow::Flow;
use crate::hub::HubServer;
use crate::recorder::Recorder;
use crb::agent::{Address, RunAgent};
use crb::runtime::InteractiveRuntime;
use serde::{Deserialize, Serialize};
use ui9::names::Fqn;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublisherInfo {
    pub fqn: Fqn,
    // TODO: Use `Class` wrapper
    pub class: String,
}

#[derive(Clone)]
pub struct Publisher<F: Flow> {
    recorder: Address<Recorder<F>>,
}

impl<F: Flow> Publisher<F> {
    pub fn new(fqn: Fqn, state: F) -> Self {
        let recorder = Recorder::new(state);
        let runtime = RunAgent::new(recorder);
        let address = runtime.address();
        if let Some(hub) = HubServer::link() {
            let info = PublisherInfo {
                fqn,
                class: F::class().into(),
            };
            hub.add_relay(info, runtime).ok();
        }
        // TODO: Send the runtime to the HUB
        Self { recorder: address }
    }

    pub fn event(&mut self, event: F::Event) {
        self.recorder.event(event).ok();
    }
}
