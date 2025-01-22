use crate::{Codec, ProtocolCodec, ProtocolData};
use anyhow::{Result, Error};
use derive_more::{AsRef, Deref, DerefMut, From};
use serde::{de::DeserializeOwned, Serialize};

/// The binary codec that uses encoding moudle.
#[derive(Debug)]
pub struct FlexCodec;

impl ProtocolCodec for FlexCodec {
    fn decode<T: ProtocolData>(data: &[u8]) -> Result<T> {
        Flex::decode(data)
    }

    fn encode<T: ProtocolData>(value: &T) -> Result<Vec<u8>> {
        Flex::encode(value)
    }
}

#[derive(Debug, From, Deref, DerefMut, AsRef)]
pub struct Flex<T>(pub T);

impl<T> Codec for Flex<T> {
    type Target = T;

    fn decode(data: &[u8]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        flexbuffers::from_slice(data).map_err(Error::from)
    }

    fn encode(value: &T) -> Result<Vec<u8>>
    where
        T: Serialize,
    {
        flexbuffers::to_vec(value).map_err(Error::from)
    }
}
