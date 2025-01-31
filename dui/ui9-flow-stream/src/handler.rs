use crate::codec::Codec;
use crate::protocol::ProtocolSet;
use libp2p::swarm::{
    handler::{ConnectionEvent, ConnectionHandler, ConnectionHandlerEvent},
    SubstreamProtocol,
};
use std::task::{Context, Poll};

pub struct Handler<TCodec>
where
    TCodec: Codec,
{
    protocol_set: ProtocolSet<TCodec::Protocol>,
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

    /// Incoming connections protocol check
    fn listen_protocol(&self) -> SubstreamProtocol<Self::InboundProtocol> {
        SubstreamProtocol::new(self.protocol_set.clone(), ())
    }

    fn connection_keep_alive(&self) -> bool {
        true
    }

    /// Outgoing connections polling
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
        log::trace!("Handler::on_connection_event: {event:?}");
    }

    fn on_behaviour_event(&mut self, request: Self::FromBehaviour) {
        todo!()
    }
}
