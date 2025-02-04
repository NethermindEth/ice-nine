use crb::core::time::{Duration, Instant};
use std::collections::VecDeque;

pub struct Queue {
    started: Instant,
    picked: Instant,
    messages: VecDeque<String>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            started: Instant::now(),
            picked: Instant::now(),
            messages: VecDeque::new(),
        }
    }

    pub fn add_message(&mut self, content: &str) {
        self.messages.push_back(content.into());
    }

    pub fn pick_next(&mut self) -> Option<&str> {
        if self.picked.elapsed() >= Duration::from_secs(1) {
            self.messages.pop_front();
            self.picked = Instant::now();
        }
        self.messages.front().map(String::as_ref)
    }
}
