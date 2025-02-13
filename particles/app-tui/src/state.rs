use crate::widgets::{Component, EventLog, JobList, PeerList, Dialog, Prompt};
use crate::layouts::AutoLayout;
use ratatui::prelude::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub struct AppState {
    pub tab_main: AutoLayout,
}

impl AppState {
    pub fn new() -> Self {
        let left_panel = AutoLayout::new(
            Direction::Vertical,
            [
                (Dialog::new().widget(), 4),
                (Prompt::new().widget(), 1),
            ],
        );
        let right_panel = AutoLayout::new(
            Direction::Vertical,
            [
                (JobList::new().widget(), 1),
                (EventLog::new().widget(), 1),
            ],
        );
        Self {
            tab_main: AutoLayout::new(
                Direction::Horizontal,
                [
                    (left_panel.widget(), 3),
                    (right_panel.widget(), 2),
                ],
            ),
        }
    }

    pub fn render(&self, f: &mut Frame<'_>) {
        self.render_dashboard(f);
    }

    pub fn render_dashboard(&self, f: &mut Frame<'_>) {
        self.tab_main.render(f.area(), f.buffer_mut());
    }
}
