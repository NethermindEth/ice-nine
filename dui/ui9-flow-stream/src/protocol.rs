use crate::codec::Protocol;
use std::convert::Infallible;
use futures::future::{ready, Ready};
use smallvec::SmallVec;
use libp2p::swarm::Stream;
use libp2p::core::upgrade::{InboundUpgrade, OutboundUpgrade, UpgradeInfo};

pub enum Event<TReq, TRes> {
    Request {
        request: TReq,
    },
    Response {
        response: TRes,
    },
}

#[derive(Debug)]
pub struct ProtocolSet<P> {
    protocols: SmallVec<[P; 2]>,
}

impl<P> UpgradeInfo for ProtocolSet<P>
where
    P: Protocol,
{
    type Info = P;
    type InfoIter = smallvec::IntoIter<[Self::Info; 2]>;

    fn protocol_info(&self) -> Self::InfoIter {
        self.protocols.clone().into_iter()
    }
}

impl<P> InboundUpgrade<Stream> for ProtocolSet<P>
where
    P: Protocol,
{
    type Output = (Stream, P);
    type Error = Infallible;
    type Future = Ready<Result<Self::Output, Self::Error>>;

    fn upgrade_inbound(self, io: Stream, protocol: Self::Info) -> Self::Future {
        ready(Ok((io, protocol)))
    }
}

impl<P> OutboundUpgrade<Stream> for ProtocolSet<P>
where
    P: Protocol,
{
    type Output = (Stream, P);
    type Error = Infallible;
    type Future = Ready<Result<Self::Output, Self::Error>>;

    fn upgrade_outbound(self, io: Stream, protocol: Self::Info) -> Self::Future {
        ready(Ok((io, protocol)))
    }
}
