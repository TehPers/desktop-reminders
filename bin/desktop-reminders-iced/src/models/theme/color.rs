use iced_native::Color;
use serde::{Deserialize, Serialize};

/// A RGBA color in a config (r, g, b, a).
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ConfigColor(pub f32, pub f32, pub f32, pub f32);

impl From<ConfigColor> for Color {
    fn from(value: ConfigColor) -> Self {
        Color::from_rgba(value.0, value.1, value.2, value.3)
    }
}
