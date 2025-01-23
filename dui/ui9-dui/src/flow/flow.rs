use super::encoding::FlowPack;
use super::packed::{PackedAction, PackedEvent, PackedState};
use anyhow::Error;
use serde::{de::DeserializeOwned, Serialize};
use ui9_codec::flex::FlexCodec;

/// Requirements for a data fraction in a data flow.
pub trait DataFraction
where
    Self: DeserializeOwned + Serialize + Clone + Send + 'static,
{
}

impl<T> DataFraction for T where T: DeserializeOwned + Serialize + Clone + Send + 'static {}

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

    /// `Event` reaction to an incoming `Action`.
    ///
    /// It expects `self` to control that process using the state.
    /// And to avoid sending responses if necessary.
    fn reaction(&self, _action: &Self::Action) -> Option<Self::Event> {
        None
    }

    /// Packs the state.
    fn pack_state(&self) -> Result<PackedState, Error> {
        FlexCodec::pack(self)
    }

    /// Unpacks the state.
    fn unpack_state(data: &PackedState) -> Result<Self, Error> {
        FlexCodec::unpack(data)
    }

    /// Packs the event.
    fn pack_event(delta: &Self::Event) -> Result<PackedEvent, Error> {
        FlexCodec::pack(delta)
    }

    /// Unpacks the event.
    fn unpack_event(data: &PackedEvent) -> Result<Self::Event, Error> {
        FlexCodec::unpack(data)
    }

    /// Packs the action.
    fn pack_action(action: &Self::Action) -> Result<PackedAction, Error> {
        FlexCodec::pack(action)
    }

    /// Unpacks the action.
    fn unpack_action(data: &PackedAction) -> Result<Self::Action, Error> {
        FlexCodec::unpack(data)
    }
}
