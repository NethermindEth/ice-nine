use crate::flow::Flow;
use crate::publisher::Tracer;
use crate::subscriber::Listener;
use derive_more::{Deref, DerefMut};
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use ui9::names::Fqn;

static PEERS: &str = "@peers";

#[derive(Deref, DerefMut)]
pub struct PeerListener {
    listener: Listener<PeerState>,
}

impl PeerListener {
    pub fn new(peer: Option<PeerId>) -> Self {
        let fqn = Fqn::root(PEERS);
        Self {
            listener: Listener::new(peer, fqn, true),
        }
    }
}

pub struct PeerTracer {
    tracer: Tracer<PeerState>,
}

impl PeerTracer {
    pub fn new() -> Self {
        let fqn = Fqn::root(PEERS);
        let state = PeerState::default();
        let tracer = Tracer::new(fqn, state);
        Self { tracer }
    }

    pub fn add_peer(&mut self, peer_id: PeerId) {
        let event = PeerEvent::AddPeer { peer_id };
        self.tracer.event(event);
    }

    pub fn del_peer(&mut self, peer_id: PeerId) {
        let event = PeerEvent::DelPeer { peer_id };
        self.tracer.event(event);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PeerEvent {
    AddPeer { peer_id: PeerId },
    DelPeer { peer_id: PeerId },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PeerState {
    pub peers: BTreeSet<PeerId>,
}

impl Flow for PeerState {
    type Event = PeerEvent;
    type Action = ();

    fn apply(&mut self, event: Self::Event) {
        match event {
            PeerEvent::AddPeer { peer_id } => {
                self.peers.insert(peer_id);
            }
            PeerEvent::DelPeer { peer_id } => {
                self.peers.remove(&peer_id);
            }
        }
    }
}
