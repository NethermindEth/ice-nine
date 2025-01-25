use crate::flow::{PackedAction, PackedEvent, PackedState};
use libp2p::request_response;
use serde::{Deserialize, Serialize};
use ui9::names::Fqn;

pub type Event = request_response::Event<Request, Response>;

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
