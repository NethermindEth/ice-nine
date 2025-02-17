use crate::flow::{Flow, Unified};
use crate::publisher::{Publisher, Tracer};
use crate::subscriber::{Listener, Subscriber};
use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use ui9::names::Fqn;

static LIMIT: usize = 10;

#[derive(Deref, DerefMut, From, Into)]
pub struct FailureSub {
    listener: Listener<Failure>,
}

impl Subscriber for Failure {
    type Driver = FailureSub;
}

#[derive(Deref, DerefMut, From, Into)]
pub struct FailurePub {
    tracer: Tracer<Failure>,
}

impl Publisher for Failure {
    type Driver = FailurePub;
}

impl Unified for Failure {
    fn fqn() -> Fqn {
        Fqn::root("@failure")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Failure {
    pub events: VecDeque<FailureData>,
}

impl Default for Failure {
    fn default() -> Self {
        Self {
            events: VecDeque::with_capacity(LIMIT + 1),
        }
    }
}

impl Flow for Failure {
    type Event = FailureData;
    type Action = FailureData;

    fn apply(&mut self, event: Self::Event) {
        self.events.push_back(event);
        if self.events.len() > LIMIT {
            self.events.pop_front();
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureData {
    pub message: String,
}
