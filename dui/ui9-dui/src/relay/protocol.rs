use crate::flow::{PackedAction, PackedEvent, PackedState};
use derive_more::From;
use serde::{Deserialize, Serialize};
use ui9::names::Fqn;

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
