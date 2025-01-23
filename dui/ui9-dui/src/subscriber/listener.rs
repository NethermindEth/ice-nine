use super::player::{Player, Ported};
use crate::hub::HubClient;
use crate::flow::Flow;
use crb::agent::{Address, RunAgent};
use crb::runtime::InteractiveRuntime;
use crb::core::watch;
use ui9::names::Fqn;

pub struct Listener<F: Flow> {
    player: Address<Player<F>>,
}


impl<F: Flow> Listener<F> {
    pub fn new(fqn: Fqn) -> Self {
        let (tx, rx) = watch::channel(Ported::Loading);
        let player = Player::new(tx);
        let runtime = RunAgent::new(player);
        let address = runtime.address();
        if let Some(hub) = HubClient::link() {
            // hub.add_relay(info, runtime).ok();
        }
        Self { player: address }
    }
}
