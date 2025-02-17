use crate::layouts::{AutoLayout, TabLayout};
use crate::widgets::{Component, Dialog, EventLog, FocusControl, JobList, Prompt, Render};
use crossterm::event::KeyEvent;
use ratatui::prelude::Direction;
use ratatui::Frame;

pub struct AppState {
    app_tabs: Box<dyn Render>,
    focus: FocusControl,
}

impl AppState {
    pub fn new() -> Self {
        let left_panel = AutoLayout::new(
            Direction::Vertical,
            [(Dialog::new().widget(), 4), (Prompt::new().widget(), 1)],
        );

        let right_panel = AutoLayout::new(
            Direction::Vertical,
            [(JobList::new().widget(), 1), (EventLog::new().widget(), 1)],
        );

        let tab_main = AutoLayout::new(
            Direction::Horizontal,
            [(left_panel.widget(), 3), (right_panel.widget(), 2)],
        )
        .widget();

        let mut focus = FocusControl::new();
        focus.set(&*tab_main);

        let app_tabs =
            TabLayout::new("Nine Dashboard".into(), [(tab_main, "Chat".to_string())]).widget();

        Self { app_tabs, focus }
    }

    pub fn render(&self, f: &mut Frame<'_>) {
        self.render_dashboard(f);
    }

    pub fn render_dashboard(&self, f: &mut Frame<'_>) {
        self.app_tabs.render(&f.area(), f.buffer_mut());
    }

    pub fn handle(&mut self, event: KeyEvent) {
        self.app_tabs.handle(event, &mut self.focus);
    }
}
