use crate::flow::{Flow, Unified};
use crate::publisher::{Publisher, Tracer};
use crate::subscriber::{Listener, Subscriber};
use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};
use ui9::names::Fqn;
use ulid::Ulid;

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OperationId {
    id: Ulid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRecord {
    pub task: String,
    pub failures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Live {
    pub operations: BTreeMap<OperationId, OperationRecord>,
    pub messages: VecDeque<String>,
}

impl Default for Live {
    fn default() -> Self {
        Self {
            operations: BTreeMap::new(),
            messages: VecDeque::with_capacity(LIMIT + 1),
        }
    }
}

impl Flow for Live {
    type Event = LiveEvent;
    type Action = LiveAction;

    fn apply(&mut self, event: Self::Event) {
        match event {
            LiveEvent::Message(message) => {
                self.messages.push_back(message);
                if self.messages.len() > LIMIT {
                    self.messages.pop_front();
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LiveEvent {
    Message(String),
}

impl From<LiveAction> for LiveEvent {
    fn from(action: LiveAction) -> Self {
        match action {
            LiveAction::Message(message) => Self::Message(message),
        }
    }
}

impl From<LiveEvent> for String {
    fn from(event: LiveEvent) -> Self {
        let LiveEvent::Message(message) = event;
        message
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LiveAction {
    Message(String),
}

impl From<&str> for LiveAction {
    fn from(message: &str) -> Self {
        Self::Message(message.into())
    }
}
