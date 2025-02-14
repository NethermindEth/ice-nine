use super::{FocusControl, Reason};
use crb::core::Unique;
use crossterm::event::KeyEvent;
use ratatui::prelude::{Alignment, Buffer, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

pub trait Component: 'static + Sized + Send {
    fn title(&self) -> Option<&str> {
        None
    }

    fn render(&self, area: Rect, buf: &mut Buffer) -> Result<(), Reason>;

    fn handle(&mut self, _event: KeyEvent, ctrl: &mut FocusControl) {}

    fn widget(self) -> Box<dyn Render> {
        Box::new(ComponentWidget {
            id: Unique::default(),
            widget: self,
        })
    }
}

pub struct ComponentWidget<C: Component> {
    id: Unique,
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
    fn id(&self) -> &Unique;
    fn render(&self, area: &Rect, buf: &mut Buffer);
    fn handle(&mut self, _event: KeyEvent, ctrl: &mut FocusControl) -> bool;
}

impl<C: Component> Render for ComponentWidget<C> {
    fn id(&self) -> &Unique {
        &self.id
    }

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

    fn handle(&mut self, event: KeyEvent, ctrl: &mut FocusControl) -> bool {
        self.widget.handle(event, ctrl);
        if ctrl.is_focused(&self.id) {
            // TODO: Call handle only if the component is in focus
            true
        } else {
            false
        }
    }
}
