use super::client::HubClient;
use super::local_player::LocalPlayer;
use super::remote_player::RemotePlayer;
use super::{Act, PlayerSetup, Ported, SubEvent};
use crate::flow::Flow;
use crate::utils::to_drainer;
use anyhow::{anyhow, Result};
use crb::agent::{RunAgent, StopRecipient};
use crb::core::{mpsc, watch};
use crb::runtime::InteractiveRuntime;
use crb::send::Sender;
use crb::superagent::Drainer;
use libp2p::PeerId;
use ui9::names::Fqn;

pub struct Listener<F: Flow> {
    player: StopRecipient<Act<F>>,
    event_rx: Option<mpsc::UnboundedReceiver<SubEvent<F>>>,
}

impl<F: Flow> Listener<F> {
    pub fn new(peer_id: Option<PeerId>, fqn: Fqn) -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let setup = PlayerSetup {
            fqn,
            state_tx: None,
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
            event_rx: Some(event_rx),
        }
    }

    pub fn events(&mut self) -> Result<Drainer<SubEvent<F>>> {
        self.event_rx
            .take()
            .map(to_drainer)
            .ok_or_else(|| anyhow!("Events stream (drainer) has taken already."))
    }

    pub fn action(&self, action: F::Action) {
        let msg = Act { action };
        self.player.send(msg).ok();
    }
}
