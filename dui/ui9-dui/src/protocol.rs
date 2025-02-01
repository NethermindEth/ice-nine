use crate::flow::{PackedAction, PackedEvent, PackedState};
use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use ui9::names::Fqn;

pub type Event = ui9_request_response::Event<Envelope<Ui9Request>, Envelope<Ui9Response>>;

#[derive(
    Debug, Serialize, Deserialize, From, Into, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy,
)]
pub struct SessionId(usize);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Envelope<T> {
    pub session_id: SessionId,
    pub value: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, From)]
pub enum Ui9Request {
    Subscribe(Fqn),
    Action(PackedAction),
    Unsubscribe,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, From)]
pub enum Ui9Response {
    State(PackedState),
    Event(PackedEvent),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, From)]
pub enum Ui9Message {
    Request(Ui9Request),
    Response(Ui9Response),
}
