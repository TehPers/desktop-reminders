use chrono::Local;
use iced_lazy::{component, Component};
use iced_native::{
    alignment::{Horizontal, Vertical},
    theme::Button,
    widget::{button, column, text, text_input, vertical_space},
    Element, Length,
};

use crate::{
    models::reminders::{
        Reminder, ReminderDaysOfWeek, ReminderFrequency, ReminderFrequencyDaily,
        ReminderFrequencyMonthly, ReminderFrequencyOnce, ReminderFrequencyWeekly,
        ReminderFrequencyYearly, ReminderMonth, ReminderTimeOfDay, ReminderYearlyDate,
    },
    ui::{
        app::Renderer,
        components::{
            config::{days_of_week, time_of_day},
            tab_list, Tab,
        },
    },
};

/// Creates a new [`AddReminderPage`].
#[inline]
pub fn add_reminder_page<'a, Message>() -> AddReminderPage<'a, Message> {
    AddReminderPage { on_add: None }
}

/// A page that allows the user to add a reminder.
#[must_use]
pub struct AddReminderPage<'a, Message> {
    on_add: Option<Box<dyn Fn(Reminder) -> Message + 'a>>,
}

impl<'a, Message> AddReminderPage<'a, Message> {
    /// Sets the function to be called when a reminder is added.
    #[inline]
    pub fn on_add<F>(mut self, f: F) -> Self
    where
        F: Fn(Reminder) -> Message + 'a,
    {
        self.on_add = Some(Box::new(f));
        self
    }
}

impl<'a, Message> Component<Message, Renderer> for AddReminderPage<'a, Message> {
    type State = AddReminderPageState;
    type Event = AddReminderPageEvent;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            AddReminderPageEvent::AddReminder => {
                let date = Local::now().date_naive();
                let frequency = match state.frequency_type {
                    FrequencyType::Once => ReminderFrequency::Once(ReminderFrequencyOnce {
                        date,
                        time: state.time_of_day,
                    }),
                    FrequencyType::Daily => ReminderFrequency::Daily(ReminderFrequencyDaily {
                        time: state.time_of_day,
                    }),
                    FrequencyType::Weekly => ReminderFrequency::Weekly(ReminderFrequencyWeekly {
                        days: ReminderDaysOfWeek::all(),
                        time: state.time_of_day,
                    }),
                    FrequencyType::Monthly => {
                        ReminderFrequency::Monthly(ReminderFrequencyMonthly {
                            dates: vec![1, 2, 8],
                            time: state.time_of_day,
                        })
                    }
                    FrequencyType::Yearly => ReminderFrequency::Yearly(ReminderFrequencyYearly {
                        dates: vec![ReminderYearlyDate {
                            month: ReminderMonth::January,
                            date: 4,
                        }],
                        time: state.time_of_day,
                    }),
                };
                let reminder = Reminder {
                    frequency,
                    message: std::mem::take(&mut state.title),
                    completed: false,
                };

                *state = Default::default();
                self.on_add.as_ref().map(|f| f(reminder))
            }
            AddReminderPageEvent::SetTitle(title) => {
                state.title = title;
                None
            }
            AddReminderPageEvent::SetFrequencyType(frequency_type) => {
                state.frequency_type = frequency_type;
                None
            }
            AddReminderPageEvent::SetDaysOfWeek(days_of_week) => {
                state.days_of_week = days_of_week;
                None
            }
            AddReminderPageEvent::SetTimeOfDay(time_of_day) => {
                state.time_of_day = time_of_day;
                None
            }
        }
    }

    fn view(&self, state: &Self::State) -> Element<'_, Self::Event, Renderer> {
        const FREQUENCY_TABS: &[(&str, FrequencyType)] = &[
            ("1", FrequencyType::Once),
            ("D", FrequencyType::Daily),
            ("W", FrequencyType::Weekly),
            ("M", FrequencyType::Monthly),
            ("Y", FrequencyType::Yearly),
        ];

        let mut rows = Vec::with_capacity(10);

        // Title
        rows.push(
            text_input("Title", &state.title)
                .on_input(AddReminderPageEvent::SetTitle)
                .width(Length::Fill)
                .into(),
        );

        // Frequency
        rows.push(
            text("Frequency")
                .width(Length::Fill)
                .height(Length::Fixed(25.0))
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Bottom)
                .into(),
        );
        rows.push(
            tab_list(
                FREQUENCY_TABS
                    .iter()
                    .copied()
                    .map(|(label, id)| Tab::new(label, id, state.frequency_type == id)),
            )
            .on_selected(|id| AddReminderPageEvent::SetFrequencyType(id))
            .into(),
        );

        // Days of week
        if state.frequency_type == FrequencyType::Weekly {
            rows.push(
                days_of_week(state.days_of_week)
                    .on_change(AddReminderPageEvent::SetDaysOfWeek)
                    .into(),
            );
        }

        // Time of day
        rows.push(
            time_of_day(state.time_of_day)
                .on_change(AddReminderPageEvent::SetTimeOfDay)
                .into(),
        );

        // Add reminder
        rows.push(vertical_space(Length::Fill).into());
        rows.push(
            button(
                text("+")
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center)
                    .width(Length::Fill),
            )
            .on_press(AddReminderPageEvent::AddReminder)
            .width(Length::Fill)
            .style(Button::Positive)
            .into(),
        );

        column(rows).into()
    }
}

impl<'a, Message> From<AddReminderPage<'a, Message>> for Element<'a, Message, Renderer>
where
    Message: 'a,
{
    fn from(value: AddReminderPage<'a, Message>) -> Self {
        component(value)
    }
}

/// An event for [`AddReminderPage`].
#[derive(Clone, Debug)]
pub enum AddReminderPageEvent {
    /// Adds a reminder.
    AddReminder,
    /// Sets the title.
    SetTitle(String),
    /// Sets the frequency type.
    SetFrequencyType(FrequencyType),
    /// Sets the days of week.
    SetDaysOfWeek(ReminderDaysOfWeek),
    /// Sets the time of day.
    SetTimeOfDay(ReminderTimeOfDay),
}

/// The state for [`AddReminderPage`].
#[derive(Clone, Debug)]
pub struct AddReminderPageState {
    title: String,
    frequency_type: FrequencyType,
    days_of_week: ReminderDaysOfWeek,
    time_of_day: ReminderTimeOfDay,
}

impl Default for AddReminderPageState {
    fn default() -> Self {
        let now = Local::now().naive_local();
        Self {
            title: Default::default(),
            frequency_type: Default::default(),
            days_of_week: Default::default(),
            time_of_day: ReminderTimeOfDay::Time { time: now.time() },
        }
    }
}

/// A kind of frequency for a [`Reminder`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub enum FrequencyType {
    /// Maps to [`ReminderFrequency::Once`].
    #[default]
    Once,
    /// Maps to [`ReminderFrequency::Daily`].
    Daily,
    /// Maps to [`ReminderFrequency::Weekly`].
    Weekly,
    /// Maps to [`ReminderFrequency::Monthly`].
    Monthly,
    /// Maps to [`ReminderFrequency::Yearly`].
    Yearly,
}
