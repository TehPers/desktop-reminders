use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
    hash::Hash,
    sync::Arc,
};

use egui::Ui;

/// A widget that wraps a configurable value.
#[derive(Debug)]
pub struct Config<'a, T>
where
    T: ?Sized + Configurable,
{
    value: &'a mut T,
    options: T::Options,
}

impl<'a, T> Config<'a, T>
where
    T: ?Sized + Configurable,
{
    /// Create a new config widget with the given options.
    #[inline]
    #[must_use]
    pub fn with_options(value: &'a mut T, options: T::Options) -> Self {
        Self { value, options }
    }

    /// Shows the config UI for the value.
    #[inline]
    pub fn show(self, id: Arc<IdTree>, ui: &mut Ui) {
        self.value.config_ui(id, ui, self.options);
    }
}

impl<'a, T> Config<'a, T>
where
    T: ?Sized + Configurable<Options = ()>,
{
    /// Create a new config widget.
    #[inline]
    #[must_use]
    pub fn new(value: &'a mut T) -> Self {
        Self::with_options(value, ())
    }
}

/// A type that can be configured via egui.
pub trait Configurable {
    /// The type of options that can be passed to the config UI.
    type Options;

    /// Configure the value via egui.
    fn config_ui(&mut self, id: Arc<IdTree>, ui: &mut Ui, options: Self::Options);

    /// Show the config UI for the value.
    fn show_config(&mut self, id: Arc<IdTree>, ui: &mut Ui)
    where
        Self: Configurable<Options = ()>,
    {
        Config::new(self).show(id, ui);
    }

    /// Show the config UI for the value with the given options.
    fn show_config_with_options(&mut self, id: Arc<IdTree>, ui: &mut Ui, options: Self::Options)
    where
        Self: Configurable,
    {
        Config::with_options(self, options).show(id, ui);
    }
}

/// A tree of ids. This is directly hashable and can be used as an id source.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct IdTree {
    parent: Option<Arc<IdTree>>,
    id: Cow<'static, str>,
}

impl IdTree {
    /// Create a new id tree.
    #[inline]
    #[must_use]
    pub fn new(id: impl Into<Cow<'static, str>>) -> Arc<Self> {
        Arc::new(Self {
            parent: None,
            id: id.into(),
        })
    }

    /// Create a new child id tree.
    #[inline]
    #[must_use]
    pub fn child(self: &Arc<Self>, child_id: impl Into<Cow<'static, str>>) -> Arc<Self> {
        Arc::new(Self {
            parent: Some(self.clone()),
            id: child_id.into(),
        })
    }
}

impl Display for IdTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(parent) = &self.parent {
            write!(f, "{parent}.")?;
        }

        write!(f, "{}", self.id)
    }
}
