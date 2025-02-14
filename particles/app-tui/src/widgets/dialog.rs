use super::markdown::MdRender;
use crate::widgets::{Component, Reason};
use n9_control_chat::{Chat, Role};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Padding, Paragraph, Widget, Wrap},
};
use ui9_app::SubState;

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
                    text.push_str(&format!("\n# 👤 Request:\n\n\n{}\n\n", msg.content));
                }
                Role::Response => {
                    text.push_str(&format!("\n# 🤖 Response:\n\n\n{}\n\n", msg.content));
                }
            }
        }
        let render = MdRender::new();
        let padding = Block::default()
            .borders(Borders::NONE)
            .padding(Padding::uniform(1));
        let paragraph = Paragraph::new(render.render(&text))
            .block(padding)
            .wrap(Wrap { trim: true });

        paragraph.render(area, buf);

        Ok(())
    }
}
