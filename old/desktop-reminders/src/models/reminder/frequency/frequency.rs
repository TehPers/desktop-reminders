use std::sync::Arc;

use chrono::Local;
use egui::{ComboBox, Ui};
use serde::{Deserialize, Serialize};

use crate::widgets::config::{Configurable, IdTree};

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
    /// Returns true if the frequency is [`ReminderFrequency::Once`].
    #[inline]
    #[must_use]
    pub const fn is_once(&self) -> bool {
        matches!(self, ReminderFrequency::Once(_))
    }

    /// Returns true if the frequency is [`ReminderFrequency::Daily`].
    #[inline]
    #[must_use]
    pub const fn is_daily(&self) -> bool {
        matches!(self, ReminderFrequency::Daily(_))
    }

    /// Returns true if the frequency is [`ReminderFrequency::Weekly`].
    #[inline]
    #[must_use]
    pub const fn is_weekly(&self) -> bool {
        matches!(self, ReminderFrequency::Weekly(_))
    }

    /// Returns true if the frequency is [`ReminderFrequency::Monthly`].
    #[inline]
    #[must_use]
    pub const fn is_monthly(&self) -> bool {
        matches!(self, ReminderFrequency::Monthly(_))
    }

    /// Returns true if the frequency is [`ReminderFrequency::Yearly`].
    #[inline]
    #[must_use]
    pub const fn is_yearly(&self) -> bool {
        matches!(self, ReminderFrequency::Yearly(_))
    }
}

impl Configurable for ReminderFrequency {
    type Options = ();

    fn config_ui(&mut self, id: Arc<IdTree>, ui: &mut Ui, _options: Self::Options) {
        ui.label("Frequency");
        ComboBox::from_id_source(id.child("frequency"))
            .selected_text(selected_text(self))
            .show_ui(ui, |ui| {
                let options: &[(&str, bool, fn() -> Self)] = &[
                    ("Once", self.is_once(), || {
                        ReminderFrequency::Once(ReminderFrequencyOnce {
                            date: Local::now().date_naive(),
                            time: ReminderTimeOfDay::AllDay,
                        })
                    }),
                    ("Daily", self.is_daily(), || {
                        ReminderFrequency::Daily(Default::default())
                    }),
                    ("Weekly", self.is_weekly(), || {
                        ReminderFrequency::Weekly(Default::default())
                    }),
                    ("Monthly", self.is_monthly(), || {
                        ReminderFrequency::Monthly(Default::default())
                    }),
                    ("Yearly", self.is_yearly(), || {
                        ReminderFrequency::Yearly(Default::default())
                    }),
                ];

                // Fill options
                for &(label, selected, constructor) in options {
                    if ui.selectable_label(selected, label).clicked() {
                        *self = constructor();
                    }
                }
            });
        ui.end_row();

        match self {
            ReminderFrequency::Once(once) => once.show_config(id.child("once"), ui),
            ReminderFrequency::Daily(daily) => daily.show_config(id.child("daily"), ui),
            ReminderFrequency::Weekly(weekly) => weekly.show_config(id.child("weekly"), ui),
            // ReminderFrequency::Monthly(monthly) => monthly.show_config("monthly", ui),
            // ReminderFrequency::Yearly(yearly) => yearly.show_config("yearly", ui),
            _ => {} // TODO
        }
    }
}

fn selected_text(frequency: &ReminderFrequency) -> &'static str {
    match frequency {
        ReminderFrequency::Once(_) => "Once",
        ReminderFrequency::Daily(_) => "Daily",
        ReminderFrequency::Weekly(_) => "Weekly",
        ReminderFrequency::Monthly(_) => "Monthly",
        ReminderFrequency::Yearly(_) => "Yearly",
    }
}
