use serde::{Deserialize, Serialize};

use crate::models::reminders::ReminderTimeOfDay;

/// A reminder frequency that occurs daily.
#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ReminderFrequencyDaily {
    /// The time of day the reminder is set for.
    pub time: ReminderTimeOfDay,
}
