use crate::widgets::{Component, Reason};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Widget},
};
use ui9_app::{Ported, PortedExt};
use ui9_dui::tracers::live::Live;
use ui9_dui::{State, Sub};

pub struct ActivityList {
    live: Sub<Live>,
    state: State<Ported<Live>>,
}

impl ActivityList {
    pub fn new() -> Self {
        let mut live = Sub::<Live>::local_unified();
        let state = live.ported_state().unwrap();
        Self { live, state }
    }
}

impl Component for ActivityList {
    fn title(&self) -> Option<&str> {
        Some("Activities")
    }

    fn render(&self, area: Rect, buf: &mut Buffer) -> Result<(), Reason> {
        let ported = self.state.borrow();
        let state = ported.state_result()?;

        let items: Vec<ListItem> = state
            .operations
            .iter()
            .map(|(_id, record)| {
                ListItem::new(Line::from(vec![Span::styled(
                    &record.task,
                    Style::default().fg(Color::White),
                )]))
            })
            .collect();

        let list = List::new(items);
        list.render(area, buf);

        for message in &state.messages {}

        Ok(())
    }
}
