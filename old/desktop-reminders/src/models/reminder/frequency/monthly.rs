use serde::{Deserialize, Serialize};

use crate::models::reminder::ReminderTimeOfDay;

/// A reminder frequency that occurs monthly.
#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ReminderFrequencyMonthly {
    /// The days of the month the reminder is set for. Some of these days might
    /// not exist.
    pub dates: Vec<u8>,
    /// The time of day the reminder is set for.
    pub time: ReminderTimeOfDay,
}
