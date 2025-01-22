//! The basic encoding routines.

use anyhow::Error;
use ui9_codec::{ProtocolCodec, ProtocolData};

/// An extension for codec to pack data for the flow.
///
/// This trait is an extension for [`ProtocolCodec`] that also converts values to and from containers.
pub trait FlowPack: ProtocolCodec {
    /// Packs the data.
    fn pack<T, P: From<Vec<u8>>>(value: &T) -> Result<P, Error>
    where
        T: ProtocolData + ?Sized,
    {
        Self::encode(value).map(P::from)
    }

    /// Unpacks the data.
    fn unpack<T, P>(v: T) -> Result<P, Error>
    where
        T: AsRef<[u8]>,
        P: ProtocolData,
    {
        Self::decode(v.as_ref())
    }
}

impl<T: ProtocolCodec> FlowPack for T {}
