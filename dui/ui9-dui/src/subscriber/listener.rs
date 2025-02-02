use super::client::HubClient;
use super::drainer;
use super::{Act, PlayerState, SubEvent};
use crate::flow::Flow;
use anyhow::{anyhow, Result};
use crb::agent::StopRecipient;
use crb::core::mpsc;
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
        let state = PlayerState {
            fqn,
            state_tx: None,
            event_tx,
        };
        let player = HubClient::spawn_player(peer_id, state);
        Self {
            player,
            event_rx: Some(event_rx),
        }
    }

    pub fn events(&mut self) -> Result<Drainer<SubEvent<F>>> {
        self.event_rx
            .take()
            .map(drainer::from_mpsc)
            .ok_or_else(|| anyhow!("Events stream (drainer) has taken already."))
    }

    pub fn action(&self, action: F::Action) {
        let msg = Act { action };
        self.player.send(msg).ok();
    }
}
