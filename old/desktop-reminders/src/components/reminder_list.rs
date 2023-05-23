use chrono::{NaiveTime, Timelike};
use desktop_reminders_core::components::{Component, ComponentContext, RequestedSize};
use egui::{Align, Layout, RichText, ScrollArea, Ui};

use crate::models::reminder::{Reminder, ReminderFrequency, ReminderTimeOfDay};

/// A list of reminders.
pub struct ReminderList<'a> {
    pub reminders: &'a [Reminder],
}

impl<'a> ReminderList<'a> {
    /// Create a new reminder list.
    pub fn new(reminders: &'a [Reminder]) -> Self {
        Self { reminders }
    }
}

impl<'a> Component for ReminderList<'a> {
    fn ui(self, ctx: ComponentContext<'_>) {
        ScrollArea::vertical().show(ctx.ui, |ui| {
            for reminder in self.reminders {
                ui.with_layout(
                    Layout::left_to_right(Align::BOTTOM).with_main_align(Align::LEFT),
                    |ui| {
                        ui.label(&reminder.message);
                        frequency(ui, &reminder.frequency);
                    },
                );
            }
        });
    }

    fn requested_size(&self) -> RequestedSize {
        todo!()
    }
}

fn frequency(ui: &mut Ui, frequency: &ReminderFrequency) {
    let time_of_day = match frequency {
        ReminderFrequency::Once(once) => &once.time,
        ReminderFrequency::Daily(daily) => &daily.time,
        ReminderFrequency::Weekly(weekly) => &weekly.time,
        ReminderFrequency::Monthly(monthly) => &monthly.time,
        ReminderFrequency::Yearly(yearly) => &yearly.time,
    };

    match time_of_day {
        ReminderTimeOfDay::AllDay => {
            ui.label(RichText::new("All day").strong());
        }
        ReminderTimeOfDay::Time { time } => {
            ui.with_layout(Layout::left_to_right(Align::BOTTOM), |ui| {
                naive_time(ui, time);
            });
        }
        ReminderTimeOfDay::TimeRange { start, end } => {
            ui.with_layout(Layout::left_to_right(Align::BOTTOM), |ui| {
                naive_time(ui, start);
                ui.label(" - ");
                naive_time(ui, end);
            });
        }
    }
}

fn naive_time(ui: &mut Ui, time: &NaiveTime) {
    let (pm, hour) = time.hour12();
    let minute = time.minute();
    let am_pm = if pm { "PM" } else { "AM" };

    ui.label(RichText::new(format!("{hour}:{minute}")).strong());
    ui.label(RichText::new(format!("{am_pm}")).small());
}
