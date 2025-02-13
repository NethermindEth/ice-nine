use crate::widgets::{Component, EventLog, JobList, PeerList};
use crate::layouts::AutoLayout;
use ratatui::prelude::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub struct AppState {
    pub tab_main: AutoLayout,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            tab_main: AutoLayout::new(
                Direction::Horizontal,
                [
                    PeerList::new().widget(),
                    AutoLayout::new(
                        Direction::Vertical,
                        [
                            JobList::new().widget(),
                            EventLog::new().widget(),
                        ],
                    ).widget(),
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
