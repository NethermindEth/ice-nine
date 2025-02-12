use crate::widgets::{Component, Reason};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Widget},
};
use ui9_app::{Ported, PortedExt};
use ui9_dui::tracers::event::Event;
use ui9_dui::{State, Sub};

pub struct EventLog {
    event: Sub<Event>,
    state: State<Ported<Event>>,
}

impl EventLog {
    pub fn new() -> Self {
        let mut event = Sub::<Event>::local_unified();
        let state = event.ported_state().unwrap();
        Self { event, state }
    }
}

impl Component for EventLog {
    fn title(&self) -> Option<&str> {
        Some("Log")
    }

    fn render(&self, area: Rect, buf: &mut Buffer) -> Result<(), Reason> {
        let ported = self.state.borrow();
        let state = ported.state_result()?;

        let items: Vec<ListItem> = state
            .events
            .iter()
            .map(|event| {
                ListItem::new(Line::from(vec![Span::styled(
                    event,
                    Style::default().fg(Color::White),
                )]))
            })
            .collect();

        let list = List::new(items);
        list.render(area, buf);

        Ok(())
    }
}
