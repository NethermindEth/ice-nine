use crate::widgets::{Component, Reason};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Widget},
};
use ui9_app::{Ported, PortedExt};
use ui9_dui::tracers::job::Job;
use ui9_dui::{State, Sub};

pub struct ActivityLog {
    job: Sub<Job>,
    state: State<Ported<Job>>,
}

impl ActivityLog {
    pub fn new() -> Self {
        let mut job = Sub::<Job>::local_unified();
        let state = job.ported_state().unwrap();
        Self { job, state }
    }
}

impl Component for ActivityLog {
    fn title(&self) -> Option<&str> {
        Some("Log")
    }

    fn render(&self, area: Rect, buf: &mut Buffer) -> Result<(), Reason> {
        let ported = self.state.borrow();
        let state = ported.state_result()?;

        let items: Vec<ListItem> = state
            .messages
            .iter()
            .map(|msg| {
                ListItem::new(Line::from(vec![Span::styled(
                    msg,
                    Style::default().fg(Color::White),
                )]))
            })
            .collect();

        let list = List::new(items);
        list.render(area, buf);

        Ok(())
    }
}
