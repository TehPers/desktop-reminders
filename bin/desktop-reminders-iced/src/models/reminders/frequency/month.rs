use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// A month of the year a reminder can be set for.
#[derive(Clone, Copy, PartialEq, Debug, Default, Serialize, Deserialize)]
pub enum ReminderMonth {
    /// January.
    #[default]
    January,
    /// February.
    February,
    /// March.
    March,
    /// April.
    April,
    /// May.
    May,
    /// June.
    June,
    /// July.
    July,
    /// August.
    August,
    /// September.
    September,
    /// October.
    October,
    /// November.
    November,
    /// December.
    December,
}

impl Display for ReminderMonth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReminderMonth::January => write!(f, "January"),
            ReminderMonth::February => write!(f, "February"),
            ReminderMonth::March => write!(f, "March"),
            ReminderMonth::April => write!(f, "April"),
            ReminderMonth::May => write!(f, "May"),
            ReminderMonth::June => write!(f, "June"),
            ReminderMonth::July => write!(f, "July"),
            ReminderMonth::August => write!(f, "August"),
            ReminderMonth::September => write!(f, "September"),
            ReminderMonth::October => write!(f, "October"),
            ReminderMonth::November => write!(f, "November"),
            ReminderMonth::December => write!(f, "December"),
        }
    }
}
