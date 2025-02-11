use derive_more::{Deref, DerefMut, From, Into};
pub use libp2p::{swarm, PeerId};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::num::ParseIntError;
use ui9::names::Fqn;
use ui9_dui::flow::{Flow, Unified};
use ui9_dui::publisher::{Publisher, Tracer};
use ui9_dui::subscriber::{Listener, Subscriber};

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
        Fqn::root("@peer")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConnectionId(u64);

// Crying about that 😭
// libp2p doesn't implement serde and doesn't allow to get a raw value.
impl TryFrom<swarm::ConnectionId> for ConnectionId {
    type Error = ParseIntError;

    fn try_from(id: swarm::ConnectionId) -> Result<Self, Self::Error> {
        id.to_string().parse().map(ConnectionId)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PeerRecord {
    pub connections: BTreeSet<ConnectionId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Peer {
    pub peers: BTreeMap<PeerId, PeerRecord>,
}

impl Flow for Peer {
    type Event = PeerEvent;
    type Action = ();

    fn apply(&mut self, event: Self::Event) {
        match event {
            PeerEvent::AddPeer { peer_id } => {
                self.peers.entry(peer_id).or_default();
            }
            PeerEvent::AddConnection {
                peer_id,
                connection,
            } => {
                self.peers
                    .entry(peer_id)
                    .or_default()
                    .connections
                    .insert(connection);
            }
            PeerEvent::DelPeer { peer_id } => {
                self.peers.remove(&peer_id);
            }
            PeerEvent::DelConnection {
                peer_id,
                connection,
            } => {
                if let Some(record) = self.peers.get_mut(&peer_id) {
                    record.connections.remove(&connection);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PeerEvent {
    AddPeer {
        peer_id: PeerId,
    },
    AddConnection {
        peer_id: PeerId,
        connection: ConnectionId,
    },
    DelPeer {
        peer_id: PeerId,
    },
    DelConnection {
        peer_id: PeerId,
        connection: ConnectionId,
    },
}
