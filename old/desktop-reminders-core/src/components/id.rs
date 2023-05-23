use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
    sync::Arc,
};

/// A component ID. This is directly hashable and can be used as an ID source.
///
/// This is a tree of IDs, with each node potentially having a parent ID.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ComponentId {
    inner: Arc<Inner>,
}

impl ComponentId {
    /// Create a new root-level component ID.
    #[inline]
    #[must_use]
    pub fn new(id: impl Into<Cow<'static, str>>) -> Self {
        let inner = Inner {
            parent: None,
            id: id.into(),
        };
        Self {
            inner: Arc::new(inner),
        }
    }

    /// Create a new child ID.
    #[inline]
    #[must_use]
    pub fn child(&self, child_id: impl Into<Cow<'static, str>>) -> Self {
        let inner = Inner {
            parent: Some(self.inner.clone()),
            id: child_id.into(),
        };
        Self {
            inner: Arc::new(inner),
        }
    }
}

impl Display for ComponentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn fmt_inner(inner: &Inner, f: &mut Formatter<'_>) -> std::fmt::Result {
            if let Some(parent) = &inner.parent {
                fmt_inner(parent, f)?;
                write!(f, "/")?;
            }

            write!(f, "{}", inner.id)
        }

        fmt_inner(&self.inner, f)
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
struct Inner {
    id: Cow<'static, str>,
    parent: Option<Arc<Inner>>,
}
