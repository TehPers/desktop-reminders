use std::{
    borrow::Cow,
    fmt::{Debug, Formatter},
};

use egui::Ui;

use super::ComponentId;

/// Context for a component.
pub struct ComponentContext<'a> {
    pub ui: &'a mut Ui,
    pub id: ComponentId,
}

impl<'a> ComponentContext<'a> {
    /// Create a new component context.
    pub fn new(ui: &'a mut Ui, id: ComponentId) -> Self {
        Self { ui, id }
    }

    /// Create a new component context with a child ID.
    pub fn child(&'a mut self, id: impl Into<Cow<'static, str>>) -> Self {
        Self::new(self.ui, self.id.child(id))
    }
}

impl<'a> Debug for ComponentContext<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentContext")
            .field("id", &self.id)
            .finish_non_exhaustive()
    }
}
