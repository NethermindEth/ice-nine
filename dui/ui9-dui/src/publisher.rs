use crate::flow::Flow;
use crate::hub::HubServer;
use crate::relay::Relay;
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
    relay: Address<Relay<F>>,
}

impl<F: Flow> Publisher<F> {
    pub fn new(fqn: Fqn, state: F) -> Self {
        let relay = Relay::new(state);
        let runtime = RunAgent::new(relay);
        let address = runtime.address();
        if let Some(hub) = HubServer::link() {
            let info = PublisherInfo {
                fqn,
                class: F::class().into(),
            };
            hub.add_relay(info, runtime).ok();
        }
        // TODO: Send the runtime to the HUB
        Self { relay: address }
    }

    pub fn event(&mut self, event: F::Event) {
        self.relay.event(event).ok();
    }
}
