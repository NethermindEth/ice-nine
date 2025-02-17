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
use ui9_tui::Spinner;

pub struct JobList {
    job: Sub<Job>,
    state: State<Ported<Job>>,
    spinner: Spinner,
}

impl JobList {
    pub fn new() -> Self {
        let mut job = Sub::<Job>::local_unified();
        let state = job.ported_state().unwrap();
        Self {
            job,
            state,
            spinner: Spinner::new(),
        }
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

        let white = Style::default().fg(Color::White);
        let blue = Style::default().fg(Color::Blue);
        let items: Vec<ListItem> = state
            .operations
            .iter()
            .map(|(_id, record)| {
                let spinner = self.spinner.spinner_char();
                ListItem::new(Line::from(vec![
                    Span::styled(spinner.to_string(), blue),
                    Span::from(" "),
                    Span::styled(&record.task, white),
                ]))
            })
            .collect();

        let list = List::new(items);
        list.render(area, buf);

        Ok(())
    }
}
