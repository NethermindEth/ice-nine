use crate::widgets::{Component, FocusControl, Reason, Render};
use crossterm::event::KeyEvent;
use ratatui::prelude::{Buffer, Constraint::*, Layout, Rect, Stylize, Widget};
use ratatui::text::Line;
use ratatui::widgets::Tabs;

pub struct TabLayout {
    title: String,
    comps: Vec<(Box<dyn Render>, String)>,
    selected: usize,
}

impl TabLayout {
    pub fn new<I>(title: String, comps: I) -> Self
    where
        I: IntoIterator<Item = (Box<dyn Render>, String)>,
    {
        Self {
            title,
            comps: comps.into_iter().collect(),
            selected: 0,
        }
    }

    fn render_title(&self, area: Rect, buf: &mut Buffer) {
        let title: &str = self.title.as_ref();
        title.bold().render(area, buf);
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = self
            .comps
            .iter()
            .map(|(_, title)| Line::from(title.as_ref()));
        Tabs::new(titles).select(self.selected).render(area, buf);
    }
}

impl Component for TabLayout {
    fn render(&self, area: Rect, buf: &mut Buffer) -> Result<(), Reason> {
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        self.render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);

        if let Some(comp) = self.comps.get(self.selected) {
            comp.0.render(&inner_area, buf);
        }

        Ok(())
    }

    fn handle(&mut self, event: KeyEvent, ctrl: &mut FocusControl) {
        for (comp, _) in &mut self.comps {
            comp.handle(event, ctrl);
        }
    }
}
