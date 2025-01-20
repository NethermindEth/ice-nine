use std::collections::BTreeMap;

pub struct AppState {
    counter: usize,
}

impl AppState {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    pub fn count_up(&mut self) {
        self.counter += 1;
    }

    pub fn frame(&self) -> AppFrame {
        let mut dashboard = BTreeMap::new();
        dashboard.insert("Counter".into(), self.counter.to_string());
        AppFrame { dashboard }
    }
}

pub struct AppFrame {
    pub dashboard: BTreeMap<String, String>,
}
