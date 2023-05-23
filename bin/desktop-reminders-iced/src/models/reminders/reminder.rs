use serde::{Deserialize, Serialize};

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
