use super::client::HubClient;
use super::local_player::{Act, LocalPlayer};
use super::Ported;
use crate::flow::Flow;
use crb::agent::{Equip, RunAgent, StopAddress};
use crb::core::watch;
use crb::runtime::InteractiveRuntime;
use libp2p::PeerId;
use std::sync::Arc;
use ui9::names::Fqn;

pub struct Listener<F: Flow> {
    player: Arc<dyn Player<F>>,
    state: watch::Receiver<Ported<F>>,
}

impl<F: Flow> Listener<F> {
    pub fn new(peer: Option<PeerId>, fqn: Fqn) -> Self {
        let (tx, rx) = watch::channel(Ported::Loading);
        let player = LocalPlayer::new(fqn.clone(), tx);
        let runtime = RunAgent::new(player);
        let address: StopAddress<LocalPlayer<F>> = runtime.address().equip();
        HubClient::add_player(runtime);
        Self {
            player: Arc::new(address),
            state: rx,
        }
    }

    pub fn action(&self, action: F::Action) {
        self.player.action(action);
    }

    pub fn state(&self) -> watch::Ref<Ported<F>> {
        self.state.borrow()
    }
}

trait Player<F: Flow>: Sync + Send {
    fn action(&self, action: F::Action);
}

impl<F: Flow> Player<F> for StopAddress<LocalPlayer<F>> {
    fn action(&self, action: F::Action) {
        let msg = Act { action };
        self.event(msg).ok();
    }
}
