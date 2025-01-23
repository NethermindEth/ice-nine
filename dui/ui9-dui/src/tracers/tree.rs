use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use ui9::names::{FlowId, Fqn};
use crate::tracer::{Tracer, TracerInfo};
use crate::flow::Flow;

pub struct Tree {
    tracer: Tracer<TreeState>,
}

impl Tree {
    pub fn new() -> Self {
        let fqn = Fqn::genesis();
        let state = TreeState::default();
        let tracer = Tracer::new(fqn, state);
        Self { tracer }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum TreeEvent {
    AddFlow { id: FlowId, info: TracerInfo },
    DelFlow { id: FlowId },
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct TreeState {
    pub root: Level,
    pub info: HashMap<FlowId, TracerInfo>,
}

impl Flow for TreeState {
    type Event = TreeEvent;
    type Action = ();

    fn apply(&mut self, event: Self::Event) {
        match event {
            TreeEvent::AddFlow { id, info } => {
                let level = self.root.discover(&info.fqn);
                level.flows.insert(id);
                self.info.insert(id, info);
            }
            TreeEvent::DelFlow { id } => {
                if let Some(info) = self.info.remove(&id) {
                    let level = self.root.discover(&info.fqn);
                    level.flows.remove(&id);
                    if level.flows.is_empty() {
                        self.root.remove(&info.fqn);
                    }
                }
            }
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
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

    pub fn remove(&mut self, fqn: &Fqn) {
        self.remove_path(fqn.as_ref());
    }

    fn remove_path(&mut self, path: &[String]) -> Option<Level> {
        if path.is_empty() {
            return None;
        }

        if path.len() == 1 {
            return self.levels.remove(&path[0]);
        }

        if let Some(level) = self.levels.get_mut(&path[0]) {
            level.remove_path(&path[1..]);
        }

        None
    }
}
