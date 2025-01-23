use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use ui9::names::{FlowId, Fqn};
use ui9_dui::Tracer;
use ui9_flow::Flow;

pub struct Tree {
    tracer: Tracer<TreeState>,
}

impl Tree {
    pub fn new() -> Self {
        let fqn = Fqn::genesis();
        let state = TreeState {};
        let tracer = Tracer::new(fqn, state);
        Self { tracer }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum TreeEvent {}

#[derive(Clone, Serialize, Deserialize)]
pub struct TreeState {}

impl Flow for TreeState {
    type Event = TreeEvent;
    type Action = ();

    fn apply(&mut self, event: Self::Event) {
        match event {}
    }
}

#[derive(Default)]
pub struct Level {
    pub levels: BTreeMap<String, Level>,
    pub flows: HashSet<FlowId>,
}

impl Level {
    pub fn discover(&mut self, fqn: &Fqn) -> &mut Level {
        let mut level = self;
        for segment in fqn.iter() {
            level = level.levels.entry(segment.into()).or_default();
        }
        level
    }
}
