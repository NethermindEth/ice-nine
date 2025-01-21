use std::collections::BTreeMap;
use uiio::protocol::RecordDe;

pub struct AppState {
    counter: usize,
    events: Vec<RecordDe>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            counter: 0,
            events: Vec::new(),
        }
    }

    pub fn count_up(&mut self) {
        self.counter += 1;
    }

    pub fn add_event(&mut self, event: RecordDe) {
        self.events.push(event);
    }

    pub fn frame(&self) -> AppFrame {
        let mut dashboard = BTreeMap::new();
        dashboard.insert("Counter".into(), self.counter.to_string());
        AppFrame {
            dashboard,
            events: self.events.clone(),
        }
    }
}

pub struct AppFrame {
    pub dashboard: BTreeMap<String, String>,
    pub events: Vec<RecordDe>,
}
