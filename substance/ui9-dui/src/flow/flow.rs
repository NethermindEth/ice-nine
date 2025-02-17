use super::encoding::FlowPack;
use super::packed::{PackedAction, PackedEvent, PackedState};
use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};
use ui9_codec::flex::FlexCodec;

/// Requirements for a data fraction in a data flow.
/// `Sync` is needed to access through the `watch` channel of the `Player`.
pub trait DataFraction
where
    Self: DeserializeOwned + Serialize + Clone + Sync + Send + 'static,
{
}

impl<T> DataFraction for T where T: DeserializeOwned + Serialize + Clone + Sync + Send + 'static {}

/// Immutable state of a data flow.
pub trait Flow: DataFraction {
    /// `ControlEvent` - that send from a client to a server
    type Action: DataFraction;

    /// `UpdateEvent` - that sent from a server to a client
    type Event: DataFraction;

    /// Generic name with pack.
    ///
    /// Used in spans and for rendering.
    fn class() -> &'static str {
        std::any::type_name::<Self>()
    }

    /// Applies the event (delta).
    fn apply(&mut self, event: Self::Event);

    /// Packs the state.
    fn pack_state(&self) -> Result<PackedState> {
        FlexCodec::pack(self)
    }

    /// Unpacks the state.
    fn unpack_state(data: &PackedState) -> Result<Self> {
        FlexCodec::unpack(data)
    }

    /// Packs the event.
    fn pack_event(delta: &Self::Event) -> Result<PackedEvent> {
        FlexCodec::pack(delta)
    }

    /// Unpacks the event.
    fn unpack_event(data: &PackedEvent) -> Result<Self::Event> {
        FlexCodec::unpack(data)
    }

    /// Packs the action.
    fn pack_action(action: &Self::Action) -> Result<PackedAction> {
        FlexCodec::pack(action)
    }

    /// Unpacks the action.
    fn unpack_action(data: &PackedAction) -> Result<Self::Action> {
        FlexCodec::unpack(data)
    }
}
