use serde::{Deserialize, Serialize};

use crate::models::reminders::{ReminderDaysOfWeek, ReminderTimeOfDay};

/// A reminder frequency that occurs weekly.
#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ReminderFrequencyWeekly {
    /// The days of the week the reminder is set for.
    pub days: ReminderDaysOfWeek,
    /// The time of day the reminder is set for.
    pub time: ReminderTimeOfDay,
}
