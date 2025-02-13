use super::Reason;
use anyhow::Error;
use ratatui::prelude::{Alignment, Buffer, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

pub trait Component: 'static + Sized + Send {
    fn title(&self) -> Option<&str> {
        None
    }

    fn render(&self, area: Rect, buf: &mut Buffer) -> Result<(), Reason>;

    fn widget(self) -> Box<dyn Render> {
        Box::new(ComponentWidget { widget: self })
    }
}

pub struct ComponentWidget<C: Component> {
    widget: C,
}

impl<C: Component> ComponentWidget<C> {
    fn render_loading(&self, area: Rect, buf: &mut Buffer, spinner: &str) {
        // Create a paragraph with the spinner animation
        let loading_text = Paragraph::new(spinner)
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center);

        // Render the widget onto the buffer
        loading_text.render(area, buf);
    }
}

pub trait Render: Send {
    fn render(&self, area: &Rect, buf: &mut Buffer);
}

impl<C: Component> Render for ComponentWidget<C> {
    fn render(&self, area: &Rect, buf: &mut Buffer) {
        let render_area = {
            if let Some(title) = self.widget.title() {
                // Create a block with borders
                let block = Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" {} ", title))
                    .style(Style::default().fg(Color::White));
                let block_inner = block.inner(*area);
                block.render(*area, buf);
                block_inner
            } else {
                *area
            }
        };

        if let Err(err) = self.widget.render(render_area, buf) {
            self.render_loading(render_area, buf, err.as_ref());
        }
    }
}
