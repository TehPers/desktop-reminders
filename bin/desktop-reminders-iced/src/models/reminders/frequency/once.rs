use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::models::reminders::ReminderTimeOfDay;

/// A reminder frequency that occurs once.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ReminderFrequencyOnce {
    /// The date the reminder is set for.
    pub date: NaiveDate,
    /// The time of day the reminder is set for.
    pub time: ReminderTimeOfDay,
}
