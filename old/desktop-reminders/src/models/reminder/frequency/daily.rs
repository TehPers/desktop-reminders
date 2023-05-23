use std::sync::Arc;

use egui::Ui;
use serde::{Deserialize, Serialize};

use crate::{
    models::reminder::ReminderTimeOfDay,
    widgets::config::{Configurable, IdTree},
};

/// A reminder frequency that occurs daily.
#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ReminderFrequencyDaily {
    /// The time of day the reminder is set for.
    pub time: ReminderTimeOfDay,
}

impl Configurable for ReminderFrequencyDaily {
    type Options = ();

    fn config_ui(&mut self, id: Arc<IdTree>, ui: &mut Ui, _options: Self::Options) {
        self.time.show_config(id.child("time"), ui);
    }
}
