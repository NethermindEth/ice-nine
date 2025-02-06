use crate::flow::{Flow, Unified};
use crate::publisher::{Publisher, Tracer};
use crate::subscriber::{Listener, Subscriber};
use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use ui9::names::Fqn;

#[derive(Deref, DerefMut, From, Into)]
pub struct LiveSub {
    listener: Listener<Live>,
}

impl Subscriber for Live {
    type Driver = LiveSub;
}

#[derive(Deref, DerefMut, From, Into)]
pub struct LivePub {
    tracer: Tracer<Live>,
}

impl Publisher for Live {
    type Driver = LivePub;
}

impl LivePub {
    pub fn add_message(&mut self, msg: &str) {}
}

impl Unified for Live {
    fn fqn() -> Fqn {
        Fqn::root("@live")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Live {}

impl Flow for Live {
    type Event = LiveEvent;
    type Action = LiveAction;

    fn apply(&mut self, _event: Self::Event) {}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveEvent(pub LiveData);

impl From<LiveAction> for LiveEvent {
    fn from(action: LiveAction) -> Self {
        Self(action.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveAction(pub LiveData);

impl From<&str> for LiveAction {
    fn from(message: &str) -> Self {
        Self(LiveData::Message {
            message: message.into(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LiveData {
    Message { message: String },
}
