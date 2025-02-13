use crate::widgets::{Component, Reason};
use n9_control_chat::{Chat, ChatEvent, Role};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Widget},
};
use ui9_app::{Ported, PortedExt, SubState};
use ui9_dui::tracers::event::Event;
use ui9_dui::{State, Sub};

pub struct Dialog {
    state: SubState<Chat>,
}

impl Dialog {
    pub fn new() -> Self {
        Self {
            state: SubState::new_local_unified(),
        }
    }
}

impl Component for Dialog {
    fn title(&self) -> Option<&str> {
        Some("Dialog")
    }

    fn render(&self, area: Rect, buf: &mut Buffer) -> Result<(), Reason> {
        let ported = self.state.borrow();
        let state = ported.state()?;

        let items: Vec<ListItem> = state
            .messages
            .iter()
            .map(|message| {
                ListItem::new(Line::from(vec![Span::styled(
                    &message.content,
                    Style::default().fg(Color::White),
                )]))
            })
            .collect();

        let list = List::new(items);
        list.render(area, buf);

        Ok(())
    }
}
