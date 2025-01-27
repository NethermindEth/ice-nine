use super::client::HubClient;
use super::local_player::LocalPlayer;
use super::remote_player::RemotePlayer;
use super::{Act, PlayerSetup, Ported};
use crate::flow::Flow;
use crb::agent::{RunAgent, StopRecipient};
use crb::core::{mpsc, watch};
use crb::runtime::InteractiveRuntime;
use crb::send::Sender;
use libp2p::PeerId;
use ui9::names::Fqn;

pub struct Listener<F: Flow> {
    player: StopRecipient<Act<F>>,
    state_rx: watch::Receiver<Ported<F>>,
    event_rx: Option<mpsc::UnboundedReceiver<F::Event>>,
}

impl<F: Flow> Listener<F> {
    pub fn new(peer_id: Option<PeerId>, fqn: Fqn, with_events: bool) -> Self {
        let (state_tx, state_rx) = watch::channel(Ported::Loading);
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let setup = PlayerSetup {
            fqn: fqn,
            state_tx,
            event_tx,
        };
        let player = {
            if let Some(peer_id) = peer_id {
                let player = RemotePlayer::new(peer_id, setup);
                let runtime = RunAgent::new(player);
                let player = runtime.address().to_stop_address().to_stop_recipient();
                HubClient::add_player(runtime);
                player
            } else {
                let player = LocalPlayer::new(setup);
                let runtime = RunAgent::new(player);
                let player = runtime.address().to_stop_address().to_stop_recipient();
                HubClient::add_player(runtime);
                player
            }
        };
        Self {
            player,
            state_rx,
            event_rx: with_events.then(|| event_rx),
        }
    }

    pub fn action(&self, action: F::Action) {
        let msg = Act { action };
        self.player.send(msg).ok();
    }

    pub fn state(&self) -> watch::Ref<Ported<F>> {
        self.state_rx.borrow()
    }
}
