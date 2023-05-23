use super::{ComponentContext, RequestedSize};

/// A component that can be rendered in the UI.
pub trait Component: Sized {
    /// Get the requested size of the widget.
    fn requested_size(&self) -> RequestedSize;

    /// Render the component in the UI.
    fn ui(self, ctx: ComponentContext<'_>);
}

/// Extension trait for components.
pub trait ComponentExt: Component {
    // /// Render the component in the UI with default options.
    // fn default_ui(self, ctx: ComponentContext<'_>)
    // where
    //     Self::Options: Default,
    // {
    //     self.ui(ctx, Default::default());
    // }
}

impl<T: Component> ComponentExt for T {}
