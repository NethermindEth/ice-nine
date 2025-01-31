use crate::protocol::ProtocolSet;
use crate::codec::Codec;
use std::task::{Context, Poll};
use libp2p::swarm::{
    handler::{ConnectionHandler, ConnectionHandlerEvent, ConnectionEvent},
    SubstreamProtocol,
};

pub struct Handler<TCodec>
where
    TCodec: Codec,
{
    codec: TCodec,
}

impl<TCodec> ConnectionHandler for Handler<TCodec>
where
    TCodec: Codec,
{
    type FromBehaviour = ();
    type ToBehaviour = ();
    type InboundProtocol = ProtocolSet<TCodec::Protocol>;
    type OutboundProtocol = ProtocolSet<TCodec::Protocol>;
    type OutboundOpenInfo = ();
    type InboundOpenInfo = ();

    fn listen_protocol(&self) -> SubstreamProtocol<Self::InboundProtocol> {
        todo!()
    }

    fn on_behaviour_event(&mut self, request: Self::FromBehaviour) {
        todo!()
    }

    fn poll(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<ConnectionHandlerEvent<ProtocolSet<TCodec::Protocol>, (), Self::ToBehaviour>> {
        todo!()
    }

    fn on_connection_event(
        &mut self,
        event: ConnectionEvent<Self::InboundProtocol, Self::OutboundProtocol>,
    ) {
        todo!()
    }
}
