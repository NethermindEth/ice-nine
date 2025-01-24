use super::player::{Act, Player, Ported};
use crate::flow::Flow;
use crate::hub::Hub;
use crb::agent::{Equip, RunAgent, StopAddress};
use crb::core::watch;
use crb::runtime::InteractiveRuntime;
use std::sync::Arc;
use ui9::names::Fqn;

pub struct Listener<F: Flow> {
    player: Arc<StopAddress<Player<F>>>,
}

impl<F: Flow> Listener<F> {
    pub fn new(fqn: Fqn) -> Self {
        let (tx, rx) = watch::channel(Ported::Loading);
        let player = Player::new(fqn, tx);
        let runtime = RunAgent::new(player);
        let address = runtime.address();
        if let Ok(hub) = Hub::link() {
            hub.client.add_player(runtime).ok();
        }
        Self {
            player: Arc::new(address.equip()),
        }
    }

    pub fn action(&self, action: F::Action) {
        let msg = Act { action };
        self.player.event(msg).ok();
    }
}
