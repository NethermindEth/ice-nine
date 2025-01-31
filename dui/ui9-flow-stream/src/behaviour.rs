use crate::handler::Handler;
use crate::protocol::Event;
use libp2p::swarm::NetworkBehaviour;

pub struct Behaviour {
}

/*
impl NetworkBehaviour for Behaviour {
    type ConnectionHandler = Handler;
    type ToSwarm = Event<Vec<u8>, Vec<u8>>;
}
*/
