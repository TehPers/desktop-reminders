use serde::{Deserialize, Serialize};

use crate::models::reminders::{ReminderMonth, ReminderTimeOfDay};

/// A reminder frequency that occurs yearly.
#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ReminderFrequencyYearly {
    /// The days of the year the reminder is set for.
    pub dates: Vec<ReminderYearlyDate>,
    /// The time of day the reminder is set for.
    pub time: ReminderTimeOfDay,
}

/// A day of the year a reminder can be set for.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ReminderYearlyDate {
    /// The month of the year the reminder is set for.
    pub month: ReminderMonth,
    /// The day of the month the reminder is set for. This day might not exist.
    pub date: u8,
}

impl Default for ReminderYearlyDate {
    fn default() -> Self {
        Self {
            month: Default::default(),
            date: 1,
        }
    }
}
