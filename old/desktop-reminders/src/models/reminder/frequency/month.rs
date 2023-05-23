use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

use egui::{ComboBox, Ui, WidgetText};
use serde::{Deserialize, Serialize};

use crate::widgets::config::{Configurable, IdTree};

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

impl From<ReminderMonth> for WidgetText {
    fn from(value: ReminderMonth) -> Self {
        value.to_string().into()
    }
}

impl Configurable for ReminderMonth {
    type Options = ();

    fn config_ui(&mut self, id: Arc<IdTree>, ui: &mut Ui, _options: Self::Options) {
        const MONTHS: &[ReminderMonth] = &[
            ReminderMonth::January,
            ReminderMonth::February,
            ReminderMonth::March,
            ReminderMonth::April,
            ReminderMonth::May,
            ReminderMonth::June,
            ReminderMonth::July,
            ReminderMonth::August,
            ReminderMonth::September,
            ReminderMonth::October,
            ReminderMonth::November,
            ReminderMonth::December,
        ];

        ComboBox::from_id_source(id.child("month"))
            .selected_text(format!("{self}"))
            .show_ui(ui, |ui| {
                for &month in MONTHS {
                    ui.selectable_value(self, month, format!("{month}"));
                }
            });
    }
}
