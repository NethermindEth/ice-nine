use crate::codec::Protocol;
use futures::future::{ready, Ready};
use libp2p::core::upgrade::{InboundUpgrade, OutboundUpgrade, UpgradeInfo};
use libp2p::swarm::Stream;
use smallvec::SmallVec;
use std::convert::Infallible;

#[derive(Debug)]
pub enum Event<TReq, TRes> {
    Request { request: TReq },
    Response { response: TRes },
}

#[derive(Debug, Clone)]
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
