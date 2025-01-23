use crate::hub::HUB;
use crate::relay::Relay;
use crb::agent::{Address, RunAgent};
use crb::runtime::InteractiveRuntime;
use serde::{Deserialize, Serialize};
use ui9::names::Fqn;
use ui9_flow::Flow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracerInfo {
    pub fqn: Fqn,
}

#[derive(Clone)]
pub struct Tracer<F: Flow> {
    relay: Address<Relay<F>>,
}

impl<F: Flow> Tracer<F> {
    pub fn new(fqn: Fqn, state: F) -> Self {
        let relay = Relay::new(state);
        let runtime = RunAgent::new(relay);
        let address = runtime.address();
        if let Some(hub) = HUB.get() {
            let info = TracerInfo { fqn };
            hub.add_relay(info, runtime).ok();
        }
        // TODO: Send the runtime to the HUB
        Self { relay: address }
    }

    pub fn event(&mut self, event: F::Event) {
        self.relay.event(event).ok();
    }
}
