use anyhow::Error;
use bytes::{BufMut, BytesMut};
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;
use tokio_util::codec::{Decoder, Encoder, LengthDelimitedCodec};

pub struct FlexCodec<T> {
    inner: LengthDelimitedCodec,
    _type: PhantomData<T>,
}

impl<T> FlexCodec<T> {
    pub fn new() -> Self {
        Self {
            inner: LengthDelimitedCodec::new(),
            _type: PhantomData,
        }
    }
}

impl<T> Decoder for FlexCodec<T>
where
    T: DeserializeOwned,
{
    type Item = T;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<T>, Self::Error> {
        if let Some(frame) = self.inner.decode(src)? {
            let item = flexbuffers::from_slice(&frame)?;
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }
}

impl<T> Encoder<T> for FlexCodec<T>
where
    T: Serialize,
{
    type Error = Error;

    fn encode(&mut self, item: T, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let cbor_bytes = flexbuffers::to_vec(&*dst)?;
        self.inner.encode(cbor_bytes.into(), dst)?;
        Ok(())
    }
}
