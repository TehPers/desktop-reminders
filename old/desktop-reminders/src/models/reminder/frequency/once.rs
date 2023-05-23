use std::sync::Arc;

use chrono::NaiveDate;
use egui::Ui;
use egui_extras::DatePickerButton;
use serde::{Deserialize, Serialize};

use crate::{
    models::reminder::ReminderTimeOfDay,
    widgets::config::{Configurable, IdTree},
};

/// A reminder frequency that occurs once.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ReminderFrequencyOnce {
    /// The date the reminder is set for.
    pub date: NaiveDate,
    /// The time of day the reminder is set for.
    pub time: ReminderTimeOfDay,
}

impl Configurable for ReminderFrequencyOnce {
    type Options = ();

    fn config_ui(&mut self, id: Arc<IdTree>, ui: &mut Ui, _options: Self::Options) {
        ui.label("Date");
        ui.add(DatePickerButton::new(&mut self.date).id_source(&id.child("date").to_string()));
        ui.end_row();
    }
}
