use crate::widgets::{Component, Reason};
use n9_control_chat::{Chat, ChatEvent, Role};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph, Widget, Wrap},
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

        let mut text = String::new();
        for msg in &state.messages {
            match msg.role {
                Role::Request => {
                    text.push_str(&format!("\n> {}\n", msg.content));
                }
                Role::Response => {
                    text.push_str(&format!("\n>> {}\n", msg.content));
                }
            }
        }
        let padding = Block::default()
            .borders(Borders::NONE)
            .padding(Padding::uniform(1));
        let paragraph = Paragraph::new(text)
            .block(padding)
            .wrap(Wrap { trim: true });

        paragraph.render(area, buf);

        Ok(())
    }
}
