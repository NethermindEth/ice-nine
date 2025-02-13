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

pub struct JobList {
    job: Sub<Job>,
    state: State<Ported<Job>>,
}

impl JobList {
    pub fn new() -> Self {
        let mut job = Sub::<Job>::local_unified();
        let state = job.ported_state().unwrap();
        Self { job, state }
    }
}

impl Component for JobList {
    fn title(&self) -> Option<&str> {
        Some("Activities")
    }

    fn render(&self, area: Rect, buf: &mut Buffer) -> Result<(), Reason> {
        let ported = self.state.borrow();
        let state = ported.state()?;

        if state.operations.is_empty() {
            return Err(r#"No active jobs ʕ•́ᴥ•̀ʔ"#.into());
        }

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

        Ok(())
    }
}
