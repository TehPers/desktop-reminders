use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Default, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ReminderTimeOfDay {
    /// The reminder is set for the whole day.
    #[default]
    AllDay,
    /// The reminder is set for a specific time of day.
    Time {
        /// The time of day the reminder is set for.
        time: NaiveTime,
    },
    /// The reminder is set for a duration of time.
    TimeRange {
        /// The start time of the duration.
        start: NaiveTime,
        /// The end time of the duration.
        end: NaiveTime,
    },
}
