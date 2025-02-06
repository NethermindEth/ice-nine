use crb::core::time::{Duration, Instant};
use std::collections::VecDeque;

static DURATION: Duration = Duration::from_millis(400);

pub struct Queue {
    picked: Option<(String, Instant)>,
    messages: VecDeque<String>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            picked: None,
            messages: VecDeque::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn add_message(&mut self, content: &str) {
        self.messages.push_back(content.into());
    }

    pub fn pick_next(&mut self) -> Option<&str> {
        let next = self.picked.as_ref().map(|(_, when)| when.elapsed() > DURATION).unwrap_or(true);
        if next {
            let next = self.messages.pop_front();
            self.picked = next.map(|msg| (msg, Instant::now()));
        }
        self.picked.as_ref().map(|(msg, _)| msg.as_str())
    }
}
