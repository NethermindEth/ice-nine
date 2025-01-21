use std::collections::BTreeMap;
use uiio::protocol::RecordDe;

#[derive(Clone)]
pub struct EventId {
    pub id: usize,
    pub info: String,
}

pub struct AppState {
    counter: usize,
    ids: Vec<EventId>,
    events: Vec<RecordDe>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            counter: 0,
            ids: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn count_up(&mut self) {
        self.counter += 1;
    }

    pub fn add_event(&mut self, event: RecordDe) {
        let id = self.events.len();
        let event_id = EventId {
            id,
            info: "Event".into(),
        };
        self.ids.push(event_id);
        self.events.push(event);
    }

    pub fn frame(&self) -> AppFrame {
        let mut dashboard = BTreeMap::new();
        dashboard.insert("Counter".into(), self.counter.to_string());
        AppFrame {
            dashboard,
            events: self.ids.clone(),
        }
    }
}

pub struct AppFrame {
    pub dashboard: BTreeMap<String, String>,
    pub events: Vec<EventId>,
}
