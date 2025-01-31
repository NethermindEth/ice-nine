use std::marker::PhantomData;
use crate::codec::Codec;
use libp2p::StreamProtocol;

pub struct Cbor<TReq, TResp> {
    phantom: PhantomData<(TReq, TResp)>,
}

impl<TReq, TResp> Codec for Cbor<TReq, TResp>
where
    TReq: Send + 'static,
    TResp: Send + 'static,
{
    type Protocol = StreamProtocol;
}
