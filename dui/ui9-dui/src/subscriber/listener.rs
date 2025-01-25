use super::client::HubClient;
use super::local_player::{Act, LocalPlayer};
use super::Ported;
use crate::flow::Flow;
use crb::agent::{Equip, RunAgent, StopAddress};
use crb::core::watch;
use crb::runtime::InteractiveRuntime;
use std::sync::Arc;
use ui9::names::Fqn;

pub struct Listener<F: Flow> {
    player: Arc<StopAddress<LocalPlayer<F>>>,
    state: watch::Receiver<Ported<F>>,
}

impl<F: Flow> Listener<F> {
    pub fn new(fqn: Fqn) -> Self {
        let (tx, rx) = watch::channel(Ported::Loading);
        let player = LocalPlayer::new(fqn.clone(), tx);
        let runtime = RunAgent::new(player);
        let address = runtime.address();
        HubClient::add_player(runtime);
        Self {
            player: Arc::new(address.equip()),
            state: rx,
        }
    }

    pub fn action(&self, action: F::Action) {
        let msg = Act { action };
        self.player.event(msg).ok();
    }

    pub fn state(&self) -> watch::Ref<Ported<F>> {
        self.state.borrow()
    }
}
