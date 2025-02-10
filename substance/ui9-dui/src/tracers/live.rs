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
    type Event = LiveData;
    type Action = LiveData;

    fn apply(&mut self, event: Self::Event) {
        match event {
            LiveData::Message(message) => {
                self.messages.push_back(message);
                if self.messages.len() > LIMIT {
                    self.messages.pop_front();
                }
            }
            LiveData::Begin { id, task } => {
                let record = OperationRecord {
                    task: task.into(),
                    failures: Vec::new(),
                };
                self.operations.insert(id, record);
            }
            LiveData::Failure { id, reason } => {
                if let Some(record) = self.operations.get_mut(&id) {
                    record.failures.push(reason);
                }
            }
            LiveData::End { id } => {
                self.operations.remove(&id);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LiveData {
    Message(String),
    Begin { id: OperationId, task: String },
    Failure { id: OperationId, reason: String },
    End { id: OperationId },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OperationId {
    id: Ulid,
}

impl OperationId {
    pub fn new() -> Self {
        Self { id: Ulid::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRecord {
    pub task: String,
    pub failures: Vec<String>,
}
