use ui9::protocol::RecordDe;

#[derive(Clone)]
pub struct EventId {
    pub id: usize,
    pub info: String,
}

pub struct AppState {
    ids: Vec<EventId>,
    events: Vec<RecordDe>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            ids: Vec::new(),
            events: Vec::new(),
        }
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
        AppFrame {
            events: self.ids.clone(),
        }
    }
}

pub struct AppFrame {
    pub events: Vec<EventId>,
}
