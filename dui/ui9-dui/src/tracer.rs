use crate::relay::Relay;
use crb::agent::{Address, RunAgent};
use crb::runtime::InteractiveRuntime;
use ui9_flow::Flow;

pub struct Tracer<F: Flow> {
    relay: Address<Relay<F>>,
}

impl<F: Flow> Tracer<F> {
    pub fn new(state: F) -> Self {
        let relay = Relay::new(state);
        let runtime = RunAgent::new(relay);
        let address = runtime.address();
        // TODO: Send the runtime to the HUB
        Self { relay: address }
    }
}
