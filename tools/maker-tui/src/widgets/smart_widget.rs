use ratatui::prelude::{Alignment, Buffer, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};
use std::borrow::Cow;

pub struct Reason {
    reason: Cow<'static, str>,
}

impl From<&'static str> for Reason {
    fn from(s: &'static str) -> Self {
        Self {
            reason: Cow::Borrowed(s),
        }
    }
}

impl AsRef<str> for Reason {
    fn as_ref(&self) -> &str {
        self.reason.as_ref()
    }
}

pub trait Component {
    fn title(&self) -> Option<&str> {
        None
    }

    fn render(&self, area: Rect, buf: &mut Buffer) -> Result<(), Reason>;
}

pub struct SmartWidget<'a, C: Component> {
    widget: &'a C,
}

impl<'a, C: Component> SmartWidget<'a, C> {
    pub fn new(widget: &'a C) -> Self {
        Self { widget }
    }
}

impl<'a, C: Component> SmartWidget<'a, C> {
    fn render_loading(&self, area: Rect, buf: &mut Buffer, spinner: &str) {
        let title = self.widget.title().unwrap_or("");
        // Create a block with borders
        let block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .style(Style::default().fg(Color::White));

        // Create a paragraph with the spinner animation
        let loading_text = Paragraph::new(spinner)
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center)
            .block(block);

        // Render the widget onto the buffer
        loading_text.render(area, buf);
    }
}

impl<'a, C: Component> Widget for SmartWidget<'a, C> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Err(err) = self.widget.render(area, buf) {
            self.render_loading(area, buf, err.as_ref());
        }
    }
}
