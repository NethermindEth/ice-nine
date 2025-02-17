use std::time::Instant;

static RATE: u64 = 200;
static CHARS: &[char] = &['⣷', '⣯', '⣟', '⡿', '⢿', '⣻', '⣽', '⣾'];

pub struct Spinner {
    started: Instant,
    rate: u64,
    chars: Box<[char]>,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            started: Instant::now(),
            rate: RATE,
            chars: CHARS.into(),
        }
    }

    pub fn spinner_char(&self) -> char {
        let elapsed = self.started.elapsed().as_millis() as u64;
        let len = self.chars.len() as u64;
        let idx = elapsed / self.rate % len;
        self.chars[idx as usize]
    }
}
