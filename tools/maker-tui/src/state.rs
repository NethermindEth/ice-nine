use crate::widgets::{PeerList, SmartWidget};
use ratatui::prelude::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, List, Paragraph};
use ratatui::Frame;

pub struct AppState {
    pub peers: PeerList,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            peers: PeerList::new(),
        }
    }

    pub fn render(&self, f: &mut Frame<'_>) {
        self.render_dashboard(f);
        /*
        let mut text = String::from("UI9 Dashboard");
        f.render_widget(text, f.area());
        */
    }

    pub fn render_dashboard(&self, f: &mut Frame<'_>) {
        // Split the screen into two columns
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(f.size());

        // Left column: Placeholder content
        let left_block = Block::default().borders(Borders::ALL).title("Left Panel");
        let left_text = Paragraph::new("This is the left panel.").block(left_block);
        f.render_widget(left_text, chunks[0]);

        // Right column: List of peers
        let widget = SmartWidget::new(&self.peers);
        f.render_widget(widget, chunks[1]);
    }
}
