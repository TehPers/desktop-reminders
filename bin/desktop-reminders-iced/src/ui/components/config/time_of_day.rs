use chrono::{Duration, Local};
use iced_aw::{time_picker::Time, TimePicker};
use iced_lazy::{component, Component};
use iced_native::{
    alignment::{Horizontal, Vertical},
    theme::Button,
    widget::{button, column, row, text, vertical_space},
    Element, Length,
};

use crate::{
    models::reminders::ReminderTimeOfDay,
    ui::{
        app::Renderer,
        components::{tab_list, Tab},
    },
};

/// Creates a new [`TimeOfDayComponent`].
#[inline]
pub fn time_of_day<'a, Message>(time_of_day: ReminderTimeOfDay) -> TimeOfDayComponent<'a, Message> {
    TimeOfDayComponent {
        time_of_day,
        on_change: None,
    }
}

/// A component that allows the user to select a time of day.
#[must_use]
pub struct TimeOfDayComponent<'a, Message> {
    time_of_day: ReminderTimeOfDay,
    on_change: Option<Box<dyn Fn(ReminderTimeOfDay) -> Message + 'a>>,
}

impl<'a, Message> TimeOfDayComponent<'a, Message> {
    /// Sets the function to be called when the time of day is changed.
    #[inline]
    pub fn on_change<F>(mut self, f: F) -> Self
    where
        F: Fn(ReminderTimeOfDay) -> Message + 'a,
    {
        self.on_change = Some(Box::new(f));
        self
    }
}

impl<'a, Message> Component<Message, Renderer> for TimeOfDayComponent<'a, Message> {
    type State = TimeOfDayComponentState;
    type Event = TimeOfDayComponentEvent;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            TimeOfDayComponentEvent::TabSelected(tab) => {
                self.time_of_day = match (tab, self.time_of_day) {
                    (TimeOfDayKind::AllDay, cur @ ReminderTimeOfDay::AllDay)
                    | (TimeOfDayKind::Time, cur @ ReminderTimeOfDay::Time { .. })
                    | (TimeOfDayKind::TimeRange, cur @ ReminderTimeOfDay::TimeRange { .. }) => cur,

                    (TimeOfDayKind::AllDay, ReminderTimeOfDay::Time { time }) => {
                        state.start = time.into();
                        ReminderTimeOfDay::AllDay
                    }
                    (TimeOfDayKind::AllDay, ReminderTimeOfDay::TimeRange { start, end }) => {
                        state.start = start.into();
                        state.end = end.into();
                        ReminderTimeOfDay::AllDay
                    }

                    (TimeOfDayKind::Time, ReminderTimeOfDay::AllDay) => ReminderTimeOfDay::Time {
                        time: state.start.into(),
                    },
                    (TimeOfDayKind::Time, ReminderTimeOfDay::TimeRange { start, end }) => {
                        state.start = start.into();
                        state.end = end.into();
                        ReminderTimeOfDay::Time { time: start }
                    }

                    (TimeOfDayKind::TimeRange, ReminderTimeOfDay::AllDay) => {
                        ReminderTimeOfDay::TimeRange {
                            start: state.start.into(),
                            end: state.end.into(),
                        }
                    }
                    (TimeOfDayKind::TimeRange, ReminderTimeOfDay::Time { time }) => {
                        state.start = time.into();
                        ReminderTimeOfDay::TimeRange {
                            start: time,
                            end: state.end.into(),
                        }
                    }
                }
            }
            TimeOfDayComponentEvent::SetStartTimeStarted => {
                state.setting_start = true;
                return None;
            }
            TimeOfDayComponentEvent::SetEndTimeStarted => {
                state.setting_end = true;
                return None;
            }
            TimeOfDayComponentEvent::SetStartTimeCancelled => {
                state.setting_start = false;
                return None;
            }
            TimeOfDayComponentEvent::SetEndTimeCancelled => {
                state.setting_end = false;
                return None;
            }
            TimeOfDayComponentEvent::SetStartTime(new_time) => {
                state.start = new_time;
                state.setting_start = false;
                match &mut self.time_of_day {
                    ReminderTimeOfDay::AllDay => {}
                    ReminderTimeOfDay::Time { time } => *time = new_time.into(),
                    ReminderTimeOfDay::TimeRange { start, .. } => *start = new_time.into(),
                }
            }
            TimeOfDayComponentEvent::SetEndTime(new_time) => {
                state.end = new_time;
                state.setting_end = false;
                match &mut self.time_of_day {
                    ReminderTimeOfDay::AllDay | ReminderTimeOfDay::Time { .. } => {}
                    ReminderTimeOfDay::TimeRange { end, .. } => *end = new_time.into(),
                }
            }
        }

        self.on_change.as_ref().map(|f| f(self.time_of_day))
    }

    fn view(&self, state: &Self::State) -> Element<'_, Self::Event, Renderer> {
        const TABS: &[(&str, TimeOfDayKind)] = &[
            ("All day", TimeOfDayKind::AllDay),
            ("Time", TimeOfDayKind::Time),
            ("Range", TimeOfDayKind::TimeRange),
        ];

        let tabs = tab_list(TABS.iter().copied().map(|(label, id)| {
            let selected = match (self.time_of_day, id) {
                (ReminderTimeOfDay::AllDay, TimeOfDayKind::AllDay) => true,
                (ReminderTimeOfDay::Time { .. }, TimeOfDayKind::Time) => true,
                (ReminderTimeOfDay::TimeRange { .. }, TimeOfDayKind::TimeRange) => true,
                _ => false,
            };

            Tab::new(label, id, selected)
        }))
        .on_selected(TimeOfDayComponentEvent::TabSelected);

        let time = match self.time_of_day {
            ReminderTimeOfDay::AllDay => vertical_space(Length::Shrink).into(),
            ReminderTimeOfDay::Time { time } => TimePicker::new(
                state.setting_start,
                time,
                button(text(time.format("%-I:%M %p")))
                    .on_press(TimeOfDayComponentEvent::SetStartTimeStarted)
                    .style(Button::Secondary),
                TimeOfDayComponentEvent::SetStartTimeCancelled,
                TimeOfDayComponentEvent::SetStartTime,
            )
            .into(),
            ReminderTimeOfDay::TimeRange { start, end } => {
                let start = TimePicker::new(
                    state.setting_start,
                    start,
                    button(text(start.format("%-I:%M %p")))
                        .on_press(TimeOfDayComponentEvent::SetStartTimeStarted)
                        .style(Button::Secondary),
                    TimeOfDayComponentEvent::SetStartTimeCancelled,
                    TimeOfDayComponentEvent::SetStartTime,
                );
                let end = TimePicker::new(
                    state.setting_end,
                    end,
                    button(text(end.format("%-I:%M %p")))
                        .on_press(TimeOfDayComponentEvent::SetEndTimeStarted)
                        .style(Button::Secondary),
                    TimeOfDayComponentEvent::SetEndTimeCancelled,
                    TimeOfDayComponentEvent::SetEndTime,
                );
                row(vec![
                    start.into(),
                    text(" - ")
                        .horizontal_alignment(Horizontal::Center)
                        .vertical_alignment(Vertical::Center)
                        .into(),
                    end.into(),
                ])
                .into()
            }
        };

        column(vec![tabs.into(), time]).into()
    }
}

impl<'a, Message> From<TimeOfDayComponent<'a, Message>> for Element<'a, Message, Renderer>
where
    Message: 'a,
{
    #[inline]
    fn from(value: TimeOfDayComponent<'a, Message>) -> Self {
        component(value)
    }
}

/// An event for [`TimeOfDayComponent`].
#[derive(Debug, Clone)]
pub enum TimeOfDayComponentEvent {
    /// A tab was selected.
    TabSelected(TimeOfDayKind),
    /// The user is starting to set the start time.
    SetStartTimeStarted,
    /// The user is starting to set the end time.
    SetEndTimeStarted,
    /// The user cancelled setting the start time.
    SetStartTimeCancelled,
    /// The user cancelled setting the end time.
    SetEndTimeCancelled,
    /// The user set the starting (or only) time.
    SetStartTime(Time),
    /// The user set the ending time.
    SetEndTime(Time),
}

/// The state for [`TimeOfDayComponent`].
#[derive(Debug, Clone)]
pub struct TimeOfDayComponentState {
    /// The starting (or only) time.
    start: Time,
    /// The ending time.
    end: Time,
    /// Whether the user is setting the starting time.
    setting_start: bool,
    /// Whether the user is setting the ending time.
    setting_end: bool,
}

impl Default for TimeOfDayComponentState {
    fn default() -> Self {
        let now = Local::now().time();
        Self {
            start: now.into(),
            end: (now + Duration::hours(1)).into(),
            setting_start: false,
            setting_end: false,
        }
    }
}

/// The kind of the time of day.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum TimeOfDayKind {
    /// Maps to [`ReminderTimeOfDay::AllDay`].
    AllDay,
    /// Maps to [`ReminderTimeOfDay::Time`].
    Time,
    /// Maps to [`ReminderTimeOfDay::TimeRange`].
    TimeRange,
}
