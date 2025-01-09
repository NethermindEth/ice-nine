use ratatui::Frame;

pub struct AppState {}

impl AppState {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, frame: &mut Frame<'_>) {
        frame.render_widget("UI9", frame.area());
    }
}
