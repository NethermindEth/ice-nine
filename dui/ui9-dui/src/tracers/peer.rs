pub use libp2p::PeerId;

use crate::flow::{Flow, Unified};
use crate::publisher::Tracer;
use crate::subscriber::Listener;
use crate::{Publisher, Subscriber};
use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use ui9::names::Fqn;

#[derive(Deref, DerefMut, From, Into)]
pub struct PeerSub {
    listener: Listener<Peer>,
}

impl Subscriber for Peer {
    type Driver = PeerSub;
}

#[derive(Deref, DerefMut, From, Into)]
pub struct PeerPub {
    tracer: Tracer<Peer>,
}

impl Publisher for Peer {
    type Driver = PeerPub;
}

impl PeerPub {
    pub fn add_peer(&mut self, peer_id: PeerId) {
        let event = PeerEvent::AddPeer { peer_id };
        self.tracer.event(event);
    }

    pub fn del_peer(&mut self, peer_id: PeerId) {
        let event = PeerEvent::DelPeer { peer_id };
        self.tracer.event(event);
    }
}

impl Unified for Peer {
    fn fqn() -> Fqn {
        Fqn::root("@peers")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Peer {
    pub peers: BTreeSet<PeerId>,
}

impl Flow for Peer {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PeerEvent {
    AddPeer { peer_id: PeerId },
    DelPeer { peer_id: PeerId },
}
