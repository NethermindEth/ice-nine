use super::player::{Player, Ported};
use crate::hub::HubClient;
use crate::flow::Flow;
use crb::agent::{Address, RunAgent, Equip, StopAddress};
use crb::runtime::InteractiveRuntime;
use crb::core::watch;
use ui9::names::Fqn;
use std::sync::Arc;

pub struct Listener<F: Flow> {
    player: Arc<StopAddress<Player<F>>>,
}


impl<F: Flow> Listener<F> {
    pub fn new(fqn: Fqn) -> Self {
        let (tx, rx) = watch::channel(Ported::Loading);
        let player = Player::new(tx);
        let runtime = RunAgent::new(player);
        let address = runtime.address();
        if let Some(hub) = HubClient::link() {
            hub.add_player(runtime).ok();
        }
        Self { player: Arc::new(address.equip()) }
    }
}
