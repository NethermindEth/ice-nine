use ratatui::prelude::{Alignment, Buffer, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

pub trait Component {
    fn render(&self, area: Rect, buf: &mut Buffer) -> Option<()>;
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
        // Create a block with borders
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Loading")
            .style(Style::default().fg(Color::Yellow));

        // Create a paragraph with the spinner animation
        let loading_text = Paragraph::new(format!("Loading {}", spinner))
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
        if self.widget.render(area, buf).is_none() {
            self.render_loading(area, buf, "...");
        }
    }
}
