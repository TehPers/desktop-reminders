use chrono::NaiveTime;
use iced_native::{
    alignment::Horizontal,
    widget::{column, container, horizontal_rule, text, vertical_space},
    Color, Command, Element, Length, Program, Theme,
};

use crate::{
    models::reminders::{
        Reminder, ReminderDaysOfWeek, ReminderFrequency, ReminderFrequencyDaily,
        ReminderFrequencyWeekly, ReminderTimeOfDay,
    },
    ui::pages::add_reminder_page,
};

use super::{
    components::{tab_list, Tab},
    pages::reminder_page,
};

/// The renderer for the app.
pub type Renderer = iced_wgpu::Renderer<Theme>;

/// The entry to the app's UI.
pub struct App {
    tab: AppTab,
    reminders: Vec<Reminder>,
}

impl App {
    /// Gets the background color of the app.
    pub fn background_color(&self) -> Color {
        Color::BLACK
    }
}

impl Default for App {
    fn default() -> Self {
        let mut reminders = vec![
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
        ];

        for idx in 1..=100 {
            reminders.push(Reminder {
                frequency: ReminderFrequency::Daily(ReminderFrequencyDaily {
                    time: ReminderTimeOfDay::AllDay,
                }),
                message: format!("Reminder {idx}"),
                completed: false,
            })
        }

        Self {
            tab: Default::default(),
            reminders,
        }
    }
}

impl Program for App {
    type Renderer = Renderer;
    type Message = AppMessage;

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMessage::TabSelected(tab) => {
                self.tab = tab;
                Command::none()
            }
            AppMessage::AddReminder(reminder) => {
                self.reminders.push(reminder);
                self.tab = AppTab::Reminders;
                Command::none()
            }
            AppMessage::ReminderToggled(index, checked) => {
                if let Some(reminder) = self.reminders.get_mut(index) {
                    reminder.completed = checked;
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message, Self::Renderer> {
        const TABS: &[(&str, AppTab)] = &[
            ("Reminders", AppTab::Reminders),
            ("New", AppTab::AddReminder),
            ("Settings", AppTab::Settings),
        ];

        let title = text("Reminders")
            .size(50)
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Center);
        let tabs = tab_list(
            TABS.iter()
                .copied()
                .map(|(label, id)| Tab::new(label, id, id == self.tab)),
        )
        .on_selected(AppMessage::TabSelected);
        let page = match self.tab {
            AppTab::Reminders => reminder_page(&self.reminders)
                .on_toggle(AppMessage::ReminderToggled)
                .into(),
            AppTab::AddReminder => add_reminder_page().on_add(AppMessage::AddReminder).into(),
            AppTab::Settings => text("Settings WIP").into(),
        };

        column(vec![
            title.into(),
            horizontal_rule(2).into(),
            container(tabs).width(Length::Fill).into(),
            page,
            vertical_space(Length::Fill).into(),
        ])
        .into()
    }
}

/// The messages that can be sent to the app.
#[derive(Clone, Debug)]
pub enum AppMessage {
    /// A tab was selected.
    TabSelected(AppTab),
    /// A reminder was added.
    AddReminder(Reminder),
    /// A reminder was toggled.
    ReminderToggled(usize, bool),
}

/// A tab in the app.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub enum AppTab {
    #[default]
    Reminders,
    AddReminder,
    Settings,
}
