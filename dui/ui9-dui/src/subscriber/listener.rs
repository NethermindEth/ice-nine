use super::client::HubClient;
use super::local_player::LocalPlayer;
use super::remote_player::RemotePlayer;
use super::{Act, Ported, TelePorted};
use crate::flow::Flow;
use crb::agent::{Equip, RunAgent, StopAddress};
use crb::core::watch;
use crb::runtime::InteractiveRuntime;
use derive_more::From;
use libp2p::PeerId;
use std::sync::Arc;
use ui9::names::Fqn;

#[derive(Debug, From)]
pub enum State<'a, F: Flow> {
    Local(watch::Ref<'a, Ported<F>>),
    Remote(watch::Ref<'a, TelePorted<F>>),
}

pub struct Listener<F: Flow> {
    player: Box<dyn ListenerInterface<F>>,
}

impl<F: Flow> Listener<F> {
    pub fn new(peer: Option<PeerId>, fqn: Fqn) -> Self {
        let player: Box<dyn ListenerInterface<F>> = {
            if let Some(peer) = peer {
                Box::new(RemoteListener::new(peer, fqn))
            } else {
                Box::new(LocalListener::new(fqn))
            }
        };
        Self { player }
    }

    pub fn state(&self) -> State<F> {
        self.player.state()
    }

    pub fn action(&self, action: F::Action) {
        self.player.action(action);
    }
}

trait ListenerInterface<F: Flow>: Sync + Send {
    fn state(&self) -> State<F>;
    fn action(&self, action: F::Action);
}

impl<F: Flow> ListenerInterface<F> for LocalListener<F> {
    fn state(&self) -> State<F> {
        State::Local(self.state.borrow())
    }

    fn action(&self, action: F::Action) {
        let msg = Act { action };
        self.player.event(msg).ok();
    }
}

impl<F: Flow> ListenerInterface<F> for RemoteListener<F> {
    fn state(&self) -> State<F> {
        State::Remote(self.state.borrow())
    }

    fn action(&self, action: F::Action) {
        let msg = Act { action };
        self.player.event(msg).ok();
    }
}

pub struct LocalListener<F: Flow> {
    player: StopAddress<LocalPlayer<F>>,
    state: watch::Receiver<Ported<F>>,
}

impl<F: Flow> LocalListener<F> {
    pub fn new(fqn: Fqn) -> Self {
        let (tx, rx) = watch::channel(Ported::Loading);
        let player = LocalPlayer::new(fqn.clone(), tx);
        let runtime = RunAgent::new(player);
        let player = runtime.address().equip();
        HubClient::add_player(runtime);
        Self { player, state: rx }
    }

    pub fn state(&self) -> watch::Ref<Ported<F>> {
        self.state.borrow()
    }
}

pub struct RemoteListener<F: Flow> {
    player: StopAddress<RemotePlayer<F>>,
    state: watch::Receiver<TelePorted<F>>,
}

impl<F: Flow> RemoteListener<F> {
    pub fn new(peer_id: PeerId, fqn: Fqn) -> Self {
        let (tx, rx) = watch::channel(TelePorted::Loading);
        let player = RemotePlayer::new(peer_id, fqn.clone(), tx);
        let runtime = RunAgent::new(player);
        let player = runtime.address().equip();
        HubClient::add_player(runtime);
        Self { player, state: rx }
    }

    pub fn state(&self) -> watch::Ref<TelePorted<F>> {
        self.state.borrow()
    }
}
