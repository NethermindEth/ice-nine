#[cfg(feature = "flex")]
pub mod flex;
#[cfg(feature = "flex")]
pub use flex::Flex;

#[cfg(feature = "toml")]
pub mod toml;
#[cfg(feature = "toml")]
pub use toml::Toml;

#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "json")]
pub use json::Json;

#[cfg(feature = "xml")]
pub mod xml;
#[cfg(feature = "xml")]
pub use xml::Xml;

use anyhow::Error;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

/// Generic procotol type.
///
/// The trait gives the following guarantees to protocol data type:
///
/// - Data can be serialized and deserialized;
/// - Instance can be sent to another thread (to be sent from a separate thread);
/// - Be printable for debugging purposes;
/// - Type has `'static` lifetime to be compatible with actor's handlers.
///
pub trait ProtocolData: Serialize + DeserializeOwned + Send + 'static {}

impl<T> ProtocolData for T where T: Serialize + DeserializeOwned + Send + 'static {}

/// The serialization format for a `Protocol`.
pub trait ProtocolCodec: Send {
    /// Decodes binary data to a type.
    fn decode<T: ProtocolData>(data: &[u8]) -> Result<T, Error>;
    /// Encodes value to a binary data.
    fn encode<T: ProtocolData>(value: &T) -> Result<Vec<u8>, Error>;
}

pub trait Codec {
    type Target;

    fn decode_self(data: &[u8]) -> Result<Self, Error>
    where
        Self::Target: DeserializeOwned,
        Self: From<Self::Target>,
    {
        Self::decode(data).map(Self::from)
    }

    fn decode(data: &[u8]) -> Result<Self::Target, Error>
    where
        Self::Target: DeserializeOwned;

    fn encode_self(&self) -> Result<Vec<u8>, Error>
    where
        Self::Target: Serialize,
        Self: AsRef<Self::Target>,
    {
        Self::encode(self.as_ref())
    }

    fn encode(value: &Self::Target) -> Result<Vec<u8>, Error>
    where
        Self::Target: Serialize;
}
