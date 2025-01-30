use crate::flow::{PackedAction, PackedEvent, PackedState};
use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use ui9::names::Fqn;

pub type Event = ui9_request_response::Event<Envelope<Request>, Envelope<Response>>;

#[derive(
    Debug, Serialize, Deserialize, From, Into, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy,
)]
pub struct SessionId(usize);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Envelope<T> {
    pub session_id: SessionId,
    pub value: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Request {
    Subscribe(Fqn),
    Action(PackedAction),
    Unsubscribe,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Response {
    State(PackedState),
    Event(PackedEvent),
}
