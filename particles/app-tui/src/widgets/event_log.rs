use crate::widgets::{Component, Reason};
use crb::core::time::Duration;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Widget},
};
use ui9_app::SubState;
use ui9_dui::tracers::event::Event;

pub struct EventLog {
    state: SubState<Event>,
}

impl EventLog {
    pub fn new() -> Self {
        Self {
            state: SubState::new_local_unified(),
        }
    }
}

impl Component for EventLog {
    fn title(&self) -> Option<&str> {
        Some("Log")
    }

    fn render(&self, area: Rect, buf: &mut Buffer) -> Result<(), Reason> {
        let ported = self.state.borrow();
        let state = ported.state()?;

        let items: Vec<ListItem> = state
            .events
            .iter()
            .map(|event| {
                ListItem::new(vec![
                    Line::from(vec![Span::styled(
                        render_duration(event.duration),
                        Style::default().fg(Color::Blue),
                    )]),
                    Line::from(vec![Span::styled(
                        &event.message,
                        Style::default().fg(Color::White),
                    )]),
                ])
            })
            .collect();

        let list = List::new(items);
        list.render(area, buf);

        Ok(())
    }
}

fn render_duration(duration: Duration) -> String {
    if duration.as_micros() < 1_000 {
        format!("{}µs", duration.as_micros())
    } else if duration.as_millis() < 1_000 {
        format!("{:.2}ms", duration.as_micros() as f32 / 1_000.0)
    } else {
        format!("{:.2}s", duration.as_millis() as f32 / 1_000.0)
    }
}
