use super::{Act, LocalPlayer, PlayerState};
use crate::flow::Flow;
use crate::relay::RemotePlayer;
use crb::agent::{RunAgent, StopRecipient};
use crb::runtime::{InteractiveRuntime, Runtime};
use libp2p::PeerId;

pub trait PlayerGenerator {
    fn new_player<F: Flow>(
        &self,
        peer_id: Option<PeerId>,
        state: PlayerState<F>,
    ) -> (Box<dyn Runtime>, StopRecipient<Act<F>>);
}

pub struct LocalGenerator;

impl PlayerGenerator for LocalGenerator {
    fn new_player<F: Flow>(
        &self,
        peer_id: Option<PeerId>,
        state: PlayerState<F>,
    ) -> (Box<dyn Runtime>, StopRecipient<Act<F>>) {
        let runtime: Box<dyn Runtime>;
        let recipient;
        if let Some(peer_id) = peer_id {
            let player = RemotePlayer::new(peer_id, state);
            let agent = RunAgent::new(player);
            recipient = agent.address().to_stop_address().to_stop_recipient();
            runtime = Box::new(agent);
        } else {
            let player = LocalPlayer::new(state);
            let agent = RunAgent::new(player);
            recipient = agent.address().to_stop_address().to_stop_recipient();
            runtime = Box::new(agent);
        }
        (runtime, recipient)
    }
}
