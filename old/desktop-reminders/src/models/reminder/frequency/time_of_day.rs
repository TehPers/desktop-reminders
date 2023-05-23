use std::sync::Arc;

use chrono::NaiveTime;
use egui::{ComboBox, Response, Ui, Widget};
use serde::{Deserialize, Serialize};

use crate::widgets::config::{Configurable, IdTree};

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
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

impl Widget for &ReminderTimeOfDay {
    fn ui(self, ui: &mut Ui) -> Response {
        match self {
            ReminderTimeOfDay::AllDay => ui.label("All day"),
            ReminderTimeOfDay::Time { time } => ui.label(time.format("%l:%M %p").to_string()),
            ReminderTimeOfDay::TimeRange { start, end } => {
                ui.horizontal(|ui| {
                    ui.label(start.format("%l:%M %p").to_string());
                    ui.label("-");
                    ui.label(end.format("%l:%M %p").to_string());
                })
                .response
            }
        }
    }
}

impl Configurable for ReminderTimeOfDay {
    type Options = ();

    fn config_ui(&mut self, id: Arc<IdTree>, ui: &mut Ui, _options: Self::Options) {
        ui.label("Time of day");
        ComboBox::from_id_source(id.child("time_of_day"))
            .selected_text(match self {
                ReminderTimeOfDay::AllDay => "All day",
                ReminderTimeOfDay::Time { .. } => "Specific time",
                ReminderTimeOfDay::TimeRange { .. } => "Time range",
            })
            .show_ui(ui, |ui| {
                let is_all_day = matches!(self, ReminderTimeOfDay::AllDay);
                let is_time = matches!(self, ReminderTimeOfDay::Time { .. });
                let is_time_range = matches!(self, ReminderTimeOfDay::TimeRange { .. });

                let all_day = ui.selectable_label(is_all_day, "All day");
                let time = ui.selectable_label(is_time, "Specific time");
                let time_range = ui.selectable_label(is_time_range, "Time range");

                if !is_all_day && all_day.clicked() {
                    *self = ReminderTimeOfDay::AllDay;
                } else if !is_time && time.clicked() {
                    *self = ReminderTimeOfDay::Time {
                        time: NaiveTime::default(),
                    };
                } else if !is_time_range && time_range.clicked() {
                    *self = ReminderTimeOfDay::TimeRange {
                        start: NaiveTime::default(),
                        end: NaiveTime::default(),
                    };
                }
            });
        ui.end_row();

        match self {
            ReminderTimeOfDay::AllDay => {}
            ReminderTimeOfDay::Time { time } => {
                ui.label("Time");
                time.show_config(id.child("time"), ui);
                ui.end_row()
            }
            ReminderTimeOfDay::TimeRange { start, end } => {
                ui.label("Start");
                start.show_config(id.child("start"), ui);
                ui.end_row();

                ui.label("End");
                end.show_config(id.child("end"), ui);
                ui.end_row();
            }
        }
    }
}
