use std::sync::Arc;

use egui::Ui;
use serde::{Deserialize, Serialize};

use crate::{
    models::reminder::{ReminderDaysOfWeek, ReminderTimeOfDay},
    widgets::config::{Configurable, IdTree},
};

/// A reminder frequency that occurs weekly.
#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ReminderFrequencyWeekly {
    /// The days of the week the reminder is set for.
    pub days: ReminderDaysOfWeek,
    /// The time of day the reminder is set for.
    pub time: ReminderTimeOfDay,
}

impl Configurable for ReminderFrequencyWeekly {
    type Options = ();

    fn config_ui(&mut self, id: Arc<IdTree>, ui: &mut Ui, _options: Self::Options) {
        self.days.show_config(id.child("days"), ui);
        self.time.show_config(id.child("time"), ui);
    }
}
