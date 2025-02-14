use crate::widgets::{Component, FocusControl, Reason};
use crossterm::event::{KeyCode, KeyEvent};
use n9_control_chat::Chat;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Padding, Paragraph, Widget, Wrap},
};
use ui9_app::SubState;

pub struct Prompt {
    state: SubState<Chat>,
    input: String,
}

impl Prompt {
    pub fn new() -> Self {
        Self {
            state: SubState::new_local_unified(),
            input: String::new(),
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
        let text = format!("{}_", self.input);
        let padding = Block::default()
            .borders(Borders::NONE)
            .padding(Padding::uniform(1));
        let input_widget = Paragraph::new(text)
            .block(padding)
            .wrap(Wrap { trim: true });
        input_widget.render(area, buf);
        Ok(())
    }

    fn handle(&mut self, event: KeyEvent, ctrl: &mut FocusControl) {
        match event.code {
            KeyCode::Char(c) => self.input.push(c),
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Enter => {
                let mut input = String::new();
                std::mem::swap(&mut self.input, &mut input);
                self.state.sub.request(input);
            }
            KeyCode::Esc => {}
            _ => {}
        }
    }
}
