use crate::widgets::{Render, Component, Reason};
use ratatui::prelude::{Buffer, Rect};

pub struct TabLayout {
    comps: Vec<(Box<dyn Render>, String)>,
}

impl TabLayout {
    pub fn new<I>(comps: I) -> Self
    where
        I: IntoIterator<Item = (Box<dyn Render>, String)>,
    {
        Self {
            comps: comps.into_iter().collect(),
        }
    }
}

impl Component for TabLayout {
    fn render(&self, area: Rect, buf: &mut Buffer) -> Result<(), Reason> {
        Ok(())
    }
}
