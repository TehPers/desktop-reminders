use std::sync::Arc;

use egui::{TextEdit, Ui};
use serde::{Deserialize, Serialize};

use crate::widgets::config::{Configurable, IdTree};

use super::ReminderFrequency;

/// A potentially recurring reminder.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Reminder {
    /// The frequency of the reminder.
    pub frequency: ReminderFrequency,
    /// The message for the reminder.
    pub message: String,
    /// Whether the reminder has been completed.
    pub completed: bool,
}

impl Configurable for Reminder {
    type Options = ();

    fn config_ui(&mut self, id: Arc<IdTree>, ui: &mut Ui, _options: Self::Options) {
        ui.label("Title");
        TextEdit::singleline(&mut self.message)
            .hint_text("Title")
            .show(ui);
        ui.end_row();

        self.frequency.show_config(id.child("frequency"), ui);
    }
}
