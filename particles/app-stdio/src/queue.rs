use crb::core::time::{Duration, Instant};
use std::collections::VecDeque;

static DURATION: Duration = Duration::from_millis(400);

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

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn add_message(&mut self, content: &str) {
        if self.messages.is_empty() {
            self.picked = Instant::now();
        }
        self.messages.push_back(content.into());
    }

    pub fn pick_next(&mut self) -> Option<&str> {
        if self.picked.elapsed() >= DURATION {
            self.messages.pop_front();
            self.picked = Instant::now();
        }
        self.messages.front().map(String::as_ref)
    }
}
