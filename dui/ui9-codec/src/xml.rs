use crate::{Codec, ProtocolCodec, ProtocolData};
use anyhow::{Error, Result};
use derive_more::{AsRef, Deref, DerefMut, From};
use serde::{de::DeserializeOwned, Serialize};

/// The binary codec that uses encoding moudle.
#[derive(Debug)]
pub struct XmlCodec;

impl ProtocolCodec for XmlCodec {
    fn decode<T: ProtocolData>(data: &[u8]) -> Result<T> {
        Xml::decode(data)
    }

    fn encode<T: ProtocolData>(value: &T) -> Result<Vec<u8>> {
        Xml::encode(value)
    }
}

#[derive(Debug, From, Deref, DerefMut, AsRef)]
pub struct Xml<T>(pub T);

impl<T> Codec for Xml<T> {
    type Target = T;

    fn decode(data: &[u8]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let data = std::str::from_utf8(data)?;
        serde_xml_rs::from_str(data).map_err(Error::from)
    }

    fn encode(value: &T) -> Result<Vec<u8>>
    where
        T: Serialize,
    {
        serde_xml_rs::to_string(value)
            .map(String::into_bytes)
            .map_err(Error::from)
    }
}
