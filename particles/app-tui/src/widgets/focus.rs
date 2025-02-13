use crate::widgets::Render;
use crb::core::Unique;

pub struct FocusControl {
    focused: Option<Unique>,
}

impl FocusControl {
    pub fn new() -> Self {
        Self { focused: None }
    }

    pub fn is_focused(&self, uq: &Unique) -> bool {
        self.focused.as_ref() == Some(uq)
    }

    pub fn set(&mut self, render: &dyn Render) {
        self.focused = Some(render.id().clone());
    }
}
