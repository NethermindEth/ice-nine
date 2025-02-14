use crate::flow::{Flow, Unified};
use crate::publisher::{Publisher, Tracer};
use crate::subscriber::{Listener, Subscriber};
use crb::core::time::Duration;
use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use ui9::names::Fqn;

static LIMIT: usize = 10;

#[derive(Deref, DerefMut, From, Into)]
pub struct EventSub {
    listener: Listener<Event>,
}

impl Subscriber for Event {
    type Driver = EventSub;
}

#[derive(Deref, DerefMut, From, Into)]
pub struct EventPub {
    tracer: Tracer<Event>,
}

impl Publisher for Event {
    type Driver = EventPub;
}

impl Unified for Event {
    fn fqn() -> Fqn {
        Fqn::root("@event")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub events: VecDeque<EventData>,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            events: VecDeque::with_capacity(LIMIT + 1),
        }
    }
}

impl Flow for Event {
    type Event = EventData;
    type Action = EventData;

    fn apply(&mut self, event: Self::Event) {
        self.events.push_back(event);
        if self.events.len() > LIMIT {
            self.events.pop_front();
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventData {
    pub message: String,
    pub duration: Duration,
}
