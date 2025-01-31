use crate::codec::Codec;
use crate::handler::Handler;
use crate::protocol::Event;
use derive_more::Display;
use libp2p::core::{transport::PortUse, Endpoint};
use libp2p::swarm::{
    behaviour::FromSwarm, ConnectionDenied, ConnectionId, NetworkBehaviour, THandler,
    THandlerInEvent, THandlerOutEvent, ToSwarm,
};
use libp2p::{Multiaddr, PeerId};
use std::task::{Context, Poll};

#[derive(Debug, Display, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OutboundRequestId(u64);

#[derive(Debug, Display, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InboundRequestId(u64);

pub struct Behaviour<TCodec> {
    codec: TCodec,
}

impl<TCodec> Behaviour<TCodec> {
    pub fn new() -> Self
    where
        TCodec: Default,
    {
        Self {
            codec: TCodec::default(),
        }
    }
}

impl<TCodec> Behaviour<TCodec>
where
    TCodec: Codec,
{
    pub fn send_request(&mut self, peer: &PeerId, request: TCodec::Request) -> OutboundRequestId {
        todo!()
    }

    pub fn send_response(&mut self, rs: TCodec::Response) -> Result<(), TCodec::Response> {
        todo!()
    }
}

impl<TCodec> NetworkBehaviour for Behaviour<TCodec>
where
    TCodec: Codec,
{
    type ConnectionHandler = Handler<TCodec>;
    type ToSwarm = Event<Vec<u8>, Vec<u8>>;

    fn handle_established_inbound_connection(
        &mut self,
        connection_id: ConnectionId,
        peer: PeerId,
        _: &Multiaddr,
        _: &Multiaddr,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        todo!()
    }

    fn handle_pending_outbound_connection(
        &mut self,
        _connection_id: ConnectionId,
        maybe_peer: Option<PeerId>,
        _addresses: &[Multiaddr],
        _effective_role: Endpoint,
    ) -> Result<Vec<Multiaddr>, ConnectionDenied> {
        todo!()
    }

    fn handle_established_outbound_connection(
        &mut self,
        connection_id: ConnectionId,
        peer: PeerId,
        remote_address: &Multiaddr,
        _: Endpoint,
        _: PortUse,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        todo!()
    }

    fn on_swarm_event(&mut self, event: FromSwarm) {}

    fn on_connection_handler_event(
        &mut self,
        peer: PeerId,
        connection_id: ConnectionId,
        event: THandlerOutEvent<Self>,
    ) {
    }

    fn poll(&mut self, _: &mut Context<'_>) -> Poll<ToSwarm<Self::ToSwarm, THandlerInEvent<Self>>> {
        todo!()
    }
}
