use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, Ordering};

pub static GENERATOR: Generator = Generator::new();

pub struct Generator {
    last_id: AtomicU32,
}

impl Generator {
    pub const fn new() -> Self {
        Self {
            last_id: AtomicU32::new(1),
        }
    }

    pub fn next(&self) -> FlowId {
        let id = self.last_id.fetch_add(1, Ordering::Relaxed);
        FlowId(id)
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Display,
)]
pub struct FlowId(u32);
