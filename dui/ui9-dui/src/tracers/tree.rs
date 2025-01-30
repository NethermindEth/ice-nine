use crate::flow::{Flow, Unified};
use crate::publisher::{Publisher, Tracer, TracerInfo};
use crate::subscriber::{Listener, Subscriber};
use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use ui9::names::Fqn;

#[derive(Deref, DerefMut, From, Into)]
pub struct TreeSub {
    listener: Listener<Tree>,
}

impl Subscriber for Tree {
    type Driver = TreeSub;
}

#[derive(Deref, DerefMut, From, Into)]
pub struct TreePub {
    tracer: Tracer<Tree>,
}

impl Publisher for Tree {
    type Driver = TreePub;
}

impl TreePub {
    pub fn add(&mut self, fqn: Fqn, info: TracerInfo) {
        let event = TreeEvent::AddFlow { fqn, info };
        self.tracer.event(event);
    }

    pub fn del(&mut self, fqn: Fqn) {
        let event = TreeEvent::DelFlow { fqn };
        self.tracer.event(event);
    }
}

impl Unified for Tree {
    fn fqn() -> Fqn {
        Fqn::root("@tree")
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum TreeEvent {
    AddFlow { fqn: Fqn, info: TracerInfo },
    DelFlow { fqn: Fqn },
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Tree {
    pub root: Level,
}

impl Flow for Tree {
    type Event = TreeEvent;
    type Action = ();

    fn apply(&mut self, event: Self::Event) {
        match event {
            TreeEvent::AddFlow { fqn, info } => {
                let level = self.root.discover(&fqn);
                level.tracer_info = Some(info);
            }
            TreeEvent::DelFlow { fqn } => {
                let level = self.root.discover(&fqn);
                level.tracer_info.take();
                self.root.remove(&fqn);
            }
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Level {
    pub levels: BTreeMap<String, Level>,
    pub tracer_info: Option<TracerInfo>,
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
