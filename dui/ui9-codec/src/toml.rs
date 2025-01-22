use crate::{Codec, ProtocolCodec, ProtocolData};
use anyhow::{Error, Result};
use derive_more::{AsRef, Deref, DerefMut, From};
use serde::{de::DeserializeOwned, Serialize};

/// The binary codec that uses encoding moudle.
#[derive(Debug)]
pub struct TomlCodec;

impl ProtocolCodec for TomlCodec {
    fn decode<T: ProtocolData>(data: &[u8]) -> Result<T> {
        Toml::decode(data)
    }

    fn encode<T: ProtocolData>(value: &T) -> Result<Vec<u8>> {
        Toml::encode(value)
    }
}

#[derive(Debug, From, Deref, DerefMut, AsRef)]
pub struct Toml<T>(pub T);

impl<T> Codec for Toml<T> {
    type Target = T;

    fn decode(data: &[u8]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let data = std::str::from_utf8(data)?;
        serde_toml::from_str(data).map_err(Error::from)
    }

    fn encode(value: &T) -> Result<Vec<u8>>
    where
        T: Serialize,
    {
        serde_toml::to_string(value)
            .map(String::into_bytes)
            .map_err(Error::from)
    }
}
