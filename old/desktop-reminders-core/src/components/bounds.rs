use egui::{Pos2, Vec2};

/// A requested widget size.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RequestedSize {
    /// The minimum size.
    pub minimum: Vec2,
    /// The maximum size.
    pub maximum: Option<Vec2>,
}

impl RequestedSize {
    /// Create a new requested size.
    pub const fn new(minimum: Vec2, maximum: Option<Vec2>) -> Self {
        Self { minimum, maximum }
    }

    /// Request any size.
    pub const fn new_unbounded() -> Self {
        Self::new(Vec2::ZERO, None)
    }

    /// Request an exact size.
    pub const fn new_exact(size: Vec2) -> Self {
        Self::new(size, Some(size))
    }

    /// Request at least a certain size.
    pub const fn new_at_least(size: Vec2) -> Self {
        Self::new(size, None)
    }
}

/// A widget bounds.
pub struct Bounds {
    /// The position of the widget.
    pub position: Pos2,
    /// The size of the widget.
    pub size: Vec2,
}

impl Bounds {
    /// Create a new bounds.
    pub const fn new(position: Pos2, size: Vec2) -> Self {
        Self { position, size }
    }
}
