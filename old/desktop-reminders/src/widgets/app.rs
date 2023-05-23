use chrono::{NaiveDate, NaiveTime};
use desktop_reminders_core::components::{Component, ComponentContext, ComponentId};
use egui::{CentralPanel, Context, FontId, Response, TextStyle, Ui, Widget};
use tracing::instrument;

use crate::{
    components::ReminderList,
    models::reminder::{
        Reminder, ReminderDaysOfWeek, ReminderFrequency, ReminderFrequencyDaily,
        ReminderFrequencyOnce, ReminderFrequencyWeekly, ReminderTimeOfDay,
    },
};

/// The main app.
#[derive(Clone, Debug)]
pub struct App {
    reminders: Vec<Reminder>,
    month: Reminder,
}

impl App {
    pub fn with_test_reminders() -> Self {
        Self {
            month: Reminder {
                frequency: ReminderFrequency::Daily(Default::default()),
                message: "Do a thing".to_string(),
                completed: false,
            },
            reminders: vec![
                Reminder {
                    frequency: ReminderFrequency::Weekly(ReminderFrequencyWeekly {
                        days: ReminderDaysOfWeek::WEEKDAYS,
                        time: ReminderTimeOfDay::AllDay,
                    }),
                    message: "Do work".to_string(),
                    completed: false,
                },
                Reminder {
                    frequency: ReminderFrequency::Weekly(ReminderFrequencyWeekly {
                        days: ReminderDaysOfWeek::WEEKDAYS,
                        time: ReminderTimeOfDay::TimeRange {
                            start: NaiveTime::from_hms_opt(12, 30, 0).unwrap(),
                            end: NaiveTime::from_hms_opt(16, 30, 0).unwrap(),
                        },
                    }),
                    message: "Dunno".to_string(),
                    completed: false,
                },
                Reminder {
                    frequency: ReminderFrequency::Once(ReminderFrequencyOnce {
                        date: NaiveDate::from_ymd_opt(2023, 2, 15).unwrap(),
                        time: ReminderTimeOfDay::Time {
                            time: NaiveTime::from_hms_opt(12, 30, 0).unwrap(),
                        },
                    }),
                    message: "Thing #1".to_string(),
                    completed: false,
                },
                Reminder {
                    frequency: ReminderFrequency::Daily(ReminderFrequencyDaily {
                        time: ReminderTimeOfDay::Time {
                            time: NaiveTime::from_hms_opt(2, 45, 0).unwrap(),
                        },
                    }),
                    message: "Thing #2".to_string(),
                    completed: false,
                },
                Reminder {
                    frequency: ReminderFrequency::Daily(ReminderFrequencyDaily {
                        time: ReminderTimeOfDay::Time {
                            time: NaiveTime::from_hms_opt(2, 45, 0).unwrap(),
                        },
                    }),
                    message: "Thing #2".to_string(),
                    completed: false,
                },
                Reminder {
                    frequency: ReminderFrequency::Daily(ReminderFrequencyDaily {
                        time: ReminderTimeOfDay::Time {
                            time: NaiveTime::from_hms_opt(2, 45, 0).unwrap(),
                        },
                    }),
                    message: "Thing #2".to_string(),
                    completed: false,
                },
            ],
        }
    }

    /// Show the app.
    #[instrument(skip_all)]
    pub fn show(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| self.ui(ui));
    }
}

impl Widget for &mut App {
    fn ui(self, ui: &mut Ui) -> Response {
        // Setup styles
        let style = ui.style_mut();
        style.text_styles = [
            (TextStyle::Heading, FontId::proportional(32.0)),
            (TextStyle::Body, FontId::proportional(24.0)),
            (TextStyle::Monospace, FontId::monospace(24.0)),
            (TextStyle::Small, FontId::proportional(12.0)),
            (TextStyle::Button, FontId::proportional(24.0)),
        ]
        .into();

        // Create UI
        let response = ui.vertical(|ui| {
            ui.heading("Reminders");
            ui.separator();

            // Show reminders
            let ctx = ComponentContext::new(ui, ComponentId::new("reminders"));
            let list = ReminderList::new(&self.reminders);
            list.ui(ctx);

            // Grid::new("config").num_columns(2).show(ui, |ui| {
            //     self.month.show_config(IdTree::new("month"), ui);
            // });

            // Add reminders
            // ScrollArea::vertical().show(ui, |ui| {
            //     let mut first = true;
            //     for reminder in &mut self.reminders {
            //         if first {
            //             first = false;
            //         } else {
            //             ui.separator();
            //         }

            //         let view = ReminderView::new(reminder);
            //         ui.add(view);
            //     }
            // });
        });

        response.response
    }
}
