use crate::flow::{Flow, Unified};
use crate::publisher::{Publisher, Tracer};
use crate::subscriber::{Listener, Subscriber};
use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::num::ParseIntError;
use ui9::names::Fqn;

#[derive(Deref, DerefMut, From, Into)]
pub struct NewsSub {
    listener: Listener<News>,
}

impl Subscriber for News {
    type Driver = NewsSub;
}

#[derive(Deref, DerefMut, From, Into)]
pub struct NewsPub {
    tracer: Tracer<News>,
}

impl Publisher for News {
    type Driver = NewsPub;
}

impl NewsPub {
    pub fn add_message(&mut self, msg: &str) {}
}

impl Unified for News {
    fn fqn() -> Fqn {
        Fqn::root("@news")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct News {}

impl Flow for News {
    type Event = NewsEvent;
    type Action = NewsAction;

    fn apply(&mut self, event: Self::Event) {}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NewsEvent {
    AddMessage { message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NewsAction {
    AddMessage { message: String },
}
