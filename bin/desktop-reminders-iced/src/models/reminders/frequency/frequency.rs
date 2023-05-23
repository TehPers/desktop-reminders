use serde::{Deserialize, Serialize};

use super::{
    ReminderFrequencyDaily, ReminderFrequencyMonthly, ReminderFrequencyOnce,
    ReminderFrequencyWeekly, ReminderFrequencyYearly, ReminderTimeOfDay,
};

/// The frequency of a reminder.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ReminderFrequency {
    /// A reminder that only occurs once.
    Once(ReminderFrequencyOnce),
    /// A reminder that occurs daily.
    Daily(ReminderFrequencyDaily),
    /// A reminder that occurs weekly.
    Weekly(ReminderFrequencyWeekly),
    /// A reminder that occurs monthly.
    Monthly(ReminderFrequencyMonthly),
    /// A reminder that occurs yearly.
    Yearly(ReminderFrequencyYearly),
}

impl ReminderFrequency {
    /// Gets the time of day that the reminder should occur.
    #[inline]
    #[must_use]
    pub const fn time_of_day(&self) -> &ReminderTimeOfDay {
        match self {
            ReminderFrequency::Once(once) => &once.time,
            ReminderFrequency::Daily(daily) => &daily.time,
            ReminderFrequency::Weekly(weekly) => &weekly.time,
            ReminderFrequency::Monthly(monthly) => &monthly.time,
            ReminderFrequency::Yearly(yearly) => &yearly.time,
        }
    }
}
