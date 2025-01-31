use crate::codec::Codec;
use libp2p::StreamProtocol;
use std::marker::PhantomData;

pub struct Cbor<TReq, TResp> {
    phantom: PhantomData<(TReq, TResp)>,
}

impl<TReq, TResp> Default for Cbor<TReq, TResp> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<TReq, TResp> Codec for Cbor<TReq, TResp>
where
    TReq: Send + 'static,
    TResp: Send + 'static,
{
    type Protocol = StreamProtocol;
    type Request = TReq;
    type Response = TResp;
}
