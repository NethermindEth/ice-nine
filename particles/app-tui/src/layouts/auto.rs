use crate::widgets::{Component, FocusControl, Reason, Render};
use crossterm::event::KeyEvent;
use ratatui::prelude::{Buffer, Constraint, Direction, Layout, Rect};

pub struct AutoLayout {
    direction: Direction,
    comps: Vec<(Box<dyn Render>, u16)>,
    cols: Vec<Constraint>,
}

impl AutoLayout {
    pub fn new<I>(direction: Direction, comps: I) -> Self
    where
        I: IntoIterator<Item = (Box<dyn Render>, u16)>,
    {
        let comps: Vec<_> = comps.into_iter().collect();
        let total: u16 = comps.iter().map(|(_, x)| x).sum();
        let point = 100 / total;
        let mut cols = Vec::new();
        for (_, size) in comps.iter() {
            let total_size = point * size;
            cols.push(Constraint::Percentage(total_size));
        }
        Self {
            direction,
            comps,
            cols,
        }
    }
}

impl Component for AutoLayout {
    fn render(&self, area: Rect, buf: &mut Buffer) -> Result<(), Reason> {
        // TODO: Replace this with a custom (grid) layout later
        let chunks = Layout::default()
            .direction(self.direction)
            .constraints(&self.cols)
            .split(area);

        let iter = self.comps.iter().zip(chunks.iter());
        for ((widget, _), chunk) in iter {
            widget.render(chunk, buf);
        }
        Ok(())
    }

    fn handle(&mut self, event: KeyEvent, ctrl: &mut FocusControl) {
        for (comp, _) in &mut self.comps {
            comp.handle(event, ctrl);
        }
    }
}
