use ratatui::widgets::Widget;
use ratatui::prelude::{Rect, Buffer, Layout, Direction, Constraint};
use crate::widgets::{Render, Component, Reason};

pub struct AutoLayout {
    direction: Direction,
    comps: Vec<Box<dyn Render>>,
    cols: Vec<Constraint>,
}

impl AutoLayout {
    pub fn new<I>(
        direction: Direction,
        comps: I,
    ) -> Self
    where I: IntoIterator<Item = Box<dyn Render>>,
    {
        let comps: Vec<_> = comps.into_iter().collect();
        let total = comps.len();
        let col = 100 / total as u16;
        let cols = std::iter::repeat(col).map(Constraint::Percentage).take(total).collect();
        Self {
            direction,
            comps,
            cols,
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        // TODO: Replace this with a custom (grid) layout later
        let chunks = Layout::default()
            .direction(self.direction)
            .constraints(&self.cols)
            .split(area);

        let iter = self.comps.iter().zip(chunks.iter());
        for (widget, chunk) in iter {
            widget.render(chunk, buf);
        }
    }
}

impl Component for AutoLayout {
    fn render(&self, area: Rect, buf: &mut Buffer) -> Result<(), Reason> {
        AutoLayout::render(self, area, buf);
        Ok(())
    }

}
