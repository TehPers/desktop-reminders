use serde::{Deserialize, Serialize};

use super::ConfigColor;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppTheme {
    background: ConfigColor,
    text: ConfigColor,
}
