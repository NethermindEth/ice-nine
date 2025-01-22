use crate::{Codec, ProtocolCodec, ProtocolData};
use anyhow::Error;
use derive_more::{AsRef, Deref, DerefMut, From};
use serde::{de::DeserializeOwned, Serialize};

/// The binary codec that uses encoding moudle.
#[derive(Debug)]
pub struct JsonCodec;

impl ProtocolCodec for JsonCodec {
    fn decode<T: ProtocolData>(data: &[u8]) -> Result<T, Error> {
        Json::decode(data)
    }

    fn encode<T: ProtocolData>(value: &T) -> Result<Vec<u8>, Error> {
        Json::encode(value)
    }
}

#[derive(Debug, From, Deref, DerefMut, AsRef)]
pub struct Json<T>(pub T);

impl<T> Codec for Json<T> {
    type Target = T;

    fn decode(data: &[u8]) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        serde_json::from_slice(data).map_err(Error::from)
    }

    fn encode(value: &T) -> Result<Vec<u8>, Error>
    where
        T: Serialize,
    {
        serde_json::to_vec(value).map_err(Error::from)
    }
}
