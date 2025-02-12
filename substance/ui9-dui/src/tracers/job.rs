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
pub struct JobSub {
    listener: Listener<Job>,
}

impl Subscriber for Job {
    type Driver = JobSub;
}

#[derive(Deref, DerefMut, From, Into)]
pub struct JobPub {
    tracer: Tracer<Job>,
}

impl Publisher for Job {
    type Driver = JobPub;
}

impl Unified for Job {
    fn fqn() -> Fqn {
        Fqn::root("@job")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub operations: BTreeMap<OperationId, OperationRecord>,
    pub messages: VecDeque<String>,
}

impl Default for Job {
    fn default() -> Self {
        Self {
            operations: BTreeMap::new(),
            messages: VecDeque::with_capacity(LIMIT + 1),
        }
    }
}

impl Flow for Job {
    type Event = JobData;
    type Action = JobData;

    fn apply(&mut self, event: Self::Event) {
        match event {
            JobData::Message(message) => {
                self.messages.push_back(message);
                if self.messages.len() > LIMIT {
                    self.messages.pop_front();
                }
            }
            JobData::Begin { id, task } => {
                let record = OperationRecord {
                    task: task.into(),
                    failures: Vec::new(),
                };
                self.operations.insert(id, record);
            }
            JobData::Failure { id, reason } => {
                if let Some(record) = self.operations.get_mut(&id) {
                    record.failures.push(reason);
                }
            }
            JobData::End { id, message } => {
                self.operations.remove(&id);
                self.messages.push_back(message);
                if self.messages.len() > LIMIT {
                    self.messages.pop_front();
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobData {
    Message(String),
    Begin { id: OperationId, task: String },
    Failure { id: OperationId, reason: String },
    End { id: OperationId, message: String },
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
