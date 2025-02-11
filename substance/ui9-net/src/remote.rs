use crate::relay::RemotePlayer;
use crb::agent::{RunAgent, StopRecipient};
use crb::runtime::{InteractiveRuntime, Runtime};
use libp2p::PeerId;
use ui9::names::Fqn;
use ui9_dui::subscriber::PlayerState;
use ui9_dui::{Act, Flow, Listener};

pub trait RemoteExt {
    fn remote(peer: PeerId, fqn: Fqn) -> Self;
}

impl<F: Flow> RemoteExt for Listener<F> {
    fn remote(peer: PeerId, fqn: Fqn) -> Self {
        todo!()
    }
}

/*
pub struct RemoteGenerator;

impl PlayerGenerator for RemoteGenerator {
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
*/
