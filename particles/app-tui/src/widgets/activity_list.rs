use crate::widgets::{Component, Reason};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Widget},
};
use ui9_dui::subscriber::State;
use ui9_dui::tracers::live::Live;

pub struct ActivityList {
    live: Option<State<Live>>,
}

impl ActivityList {
    pub fn new() -> Self {
        Self { live: None }
    }

    pub fn set_state(&mut self, live: State<Live>) {
        self.live = Some(live);
    }
}

impl Component for ActivityList {
    fn title(&self) -> Option<&str> {
        Some("Activities")
    }

    fn render(&self, area: Rect, buf: &mut Buffer) -> Result<(), Reason> {
        Ok(())
    }
}
