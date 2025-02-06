use crate::flow::{Flow, Unified};
use crate::publisher::{Publisher, Tracer};
use crate::subscriber::{Listener, Subscriber};
use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use ui9::names::Fqn;

static LIMIT: usize = 10;

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

impl Unified for Live {
    fn fqn() -> Fqn {
        Fqn::root("@live")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Live {
    pub messages: VecDeque<String>,
}

impl Default for Live {
    fn default() -> Self {
        Self {
            messages: VecDeque::with_capacity(LIMIT + 1),
        }
    }
}

impl Flow for Live {
    type Event = LiveEvent;
    type Action = LiveAction;

    fn apply(&mut self, event: Self::Event) {
        let LiveData::Message { message } = event.0;
        self.messages.push_back(message);
        if self.messages.len() > LIMIT {
            self.messages.pop_front();
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveEvent(pub LiveData);

impl From<LiveAction> for LiveEvent {
    fn from(action: LiveAction) -> Self {
        Self(action.0)
    }
}

impl From<LiveEvent> for String {
    fn from(event: LiveEvent) -> Self {
        let LiveData::Message { message } = event.0;
        message
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
