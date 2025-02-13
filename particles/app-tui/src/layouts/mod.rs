use ratatui::widgets::Widget;
use ratatui::prelude::{Rect, Buffer, Layout, Direction, Constraint};
use crate::widgets::Render;

pub struct AutoLayout {
    comps: Vec<Box<dyn Render>>,
    cols: Vec<Constraint>,
}

impl AutoLayout {
    pub fn new(comps: impl IntoIterator<Item = Box<dyn Render>>) -> Self {
        let comps: Vec<_> = comps.into_iter().collect();
        let total = comps.len();
        let col = 100 / total as u16;
        let cols = std::iter::repeat(col).map(Constraint::Percentage).take(total).collect();
        Self {
            comps,
            cols,
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        // TODO: Replace this with a custom (grid) layout later
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(&self.cols)
            .split(area);

        let iter = self.comps.iter().zip(chunks.iter());
        for (widget, chunk) in iter {
            widget.render(chunk, buf);
        }
    }
}
