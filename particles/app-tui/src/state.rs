use crate::widgets::{ActivityList, Component, EventLog, PeerList};
use ratatui::prelude::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub struct AppState {
    pub peers: PeerList,
    pub activity_list: ActivityList,
    pub event_log: EventLog,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            peers: PeerList::new(),
            activity_list: ActivityList::new(),
            event_log: EventLog::new(),
        }
    }

    pub fn render(&self, f: &mut Frame<'_>) {
        self.render_dashboard(f);
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

        let vchunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
            .split(chunks[1]);

        // Center column: Activities
        let widget = self.activity_list.widget();
        f.render_widget(widget, vchunks[0]);

        let widget = self.event_log.widget();
        f.render_widget(widget, vchunks[1]);

        // Right column: List of peers
        let widget = self.peers.widget();
        f.render_widget(widget, chunks[2]);
    }
}
