use crate::widgets::{ActivityList, Component, PeerList};
use ratatui::prelude::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub struct AppState {
    pub peers: PeerList,
    pub activity: ActivityList,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            peers: PeerList::new(),
            activity: ActivityList::new(),
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
            .constraints(
                [
                    Constraint::Percentage(40),
                    Constraint::Percentage(30),
                    Constraint::Percentage(30),
                ]
                .as_ref(),
            )
            .split(f.area());

        // Left column: Placeholder content
        let left_block = Block::default().borders(Borders::ALL).title("Left Panel");
        let left_text = Paragraph::new("This is the left panel.").block(left_block);
        f.render_widget(left_text, chunks[0]);

        // Center column: Activities
        let widget = self.activity.widget();
        f.render_widget(widget, chunks[1]);

        // Right column: List of peers
        let widget = self.peers.widget();
        f.render_widget(widget, chunks[2]);
    }
}
