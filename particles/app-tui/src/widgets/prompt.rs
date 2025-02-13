use crate::widgets::{Component, Reason};
use n9_control_chat::{Chat, ChatEvent, Role};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph, Widget},
};
use ui9_app::{Ported, PortedExt, SubState};
use ui9_dui::tracers::event::Event;
use ui9_dui::{State, Sub};

pub struct Prompt {
    state: SubState<Chat>,
    text: String,
}

impl Prompt {
    pub fn new() -> Self {
        Self {
            state: SubState::new_local_unified(),
            text: String::new(),
        }
    }
}

impl Component for Prompt {
    fn title(&self) -> Option<&str> {
        Some("Prompt")
    }

    fn render(&self, area: Rect, buf: &mut Buffer) -> Result<(), Reason> {
        let ported = self.state.borrow();
        let state = ported.state()?;

        // TODO: Show the placeholder here
        let input_widget = Paragraph::new(&*self.text).block(
            Block::default()
                .borders(Borders::NONE)
                .padding(Padding::uniform(1)),
        );
        input_widget.render(area, buf);
        Ok(())
    }
}
