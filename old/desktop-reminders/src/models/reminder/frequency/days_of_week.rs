use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

use bitflags::bitflags;
use egui::Ui;
use serde::{Deserialize, Serialize};

use crate::widgets::config::{Configurable, IdTree};

bitflags! {
    /// Days of the week a reminder can be set for.
    #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default, Serialize, Deserialize)]
    pub struct ReminderDaysOfWeek: u8 {
        /// Monday.
        const MONDAY = 1 << 0;
        /// Tuesday.
        const TUESDAY = 1 << 1;
        /// Wednesday.
        const WEDNESDAY = 1 << 2;
        /// Thursday.
        const THURSDAY = 1 << 3;
        /// Friday.
        const FRIDAY = 1 << 4;
        /// Saturday.
        const SATURDAY = 1 << 5;
        /// Sunday.
        const SUNDAY = 1 << 6;

        /// Weekdays.
        const WEEKDAYS = Self::MONDAY.bits() | Self::TUESDAY.bits() | Self::WEDNESDAY.bits() | Self::THURSDAY.bits() | Self::FRIDAY.bits();
        /// Weekends.
        const WEEKENDS = Self::SATURDAY.bits() | Self::SUNDAY.bits();
        /// Monday, Wednesday, and Friday.
        const MWF = Self::MONDAY.bits() | Self::WEDNESDAY.bits() | Self::FRIDAY.bits();
        /// Tuesday and Thursday.
        const TTH = Self::TUESDAY.bits() | Self::THURSDAY.bits();
    }
}

impl Display for ReminderDaysOfWeek {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if *self == ReminderDaysOfWeek::WEEKDAYS {
            return write!(f, "Weekdays");
        }
        if *self == ReminderDaysOfWeek::WEEKENDS {
            return write!(f, "Weekends");
        }
        if *self == ReminderDaysOfWeek::MWF {
            return write!(f, "MWF");
        }
        if *self == ReminderDaysOfWeek::TTH {
            return write!(f, "TTH");
        }

        const DAYS: &[(ReminderDaysOfWeek, &str)] = &[
            (ReminderDaysOfWeek::MONDAY, "Mon"),
            (ReminderDaysOfWeek::TUESDAY, "Tue"),
            (ReminderDaysOfWeek::WEDNESDAY, "Wed"),
            (ReminderDaysOfWeek::THURSDAY, "Thu"),
            (ReminderDaysOfWeek::FRIDAY, "Fri"),
            (ReminderDaysOfWeek::SATURDAY, "Sat"),
            (ReminderDaysOfWeek::SUNDAY, "Sun"),
        ];

        let mut first = true;
        for &(day, name) in DAYS {
            if self.contains(day) {
                if first {
                    first = false;
                } else {
                    write!(f, ", ")?;
                }
                write!(f, "{}", name)?;
            }
        }

        Ok(())
    }
}

impl Configurable for ReminderDaysOfWeek {
    type Options = ();

    fn config_ui(&mut self, _id: Arc<IdTree>, ui: &mut Ui, _options: Self::Options) {
        const DAYS: &[(ReminderDaysOfWeek, &str)] = &[
            (ReminderDaysOfWeek::MONDAY, "Mon"),
            (ReminderDaysOfWeek::TUESDAY, "Tue"),
            (ReminderDaysOfWeek::WEDNESDAY, "Wed"),
            (ReminderDaysOfWeek::THURSDAY, "Thu"),
            (ReminderDaysOfWeek::FRIDAY, "Fri"),
            (ReminderDaysOfWeek::SATURDAY, "Sat"),
            (ReminderDaysOfWeek::SUNDAY, "Sun"),
        ];

        ui.label("Days of the week");
        ui.horizontal(|ui| {
            for &(day, label) in DAYS {
                if ui.selectable_label(self.contains(day), label).clicked() {
                    self.toggle(day);
                }
            }
        });
        ui.end_row();
    }
}
