use iced_lazy::{component, Component};
use iced_native::{
    alignment::{Horizontal, Vertical},
    theme::Button,
    widget::{button, row, text},
    Element, Length,
};

use crate::{models::reminders::ReminderDaysOfWeek, ui::app::Renderer};

/// Creates a new [`DaysOfWeekComponent`].
#[inline]
pub fn days_of_week<'a, Message>(value: ReminderDaysOfWeek) -> DaysOfWeekComponent<'a, Message> {
    DaysOfWeekComponent {
        value,
        on_change: None,
    }
}

/// A component that allows the user to select days of the week.
#[must_use]
pub struct DaysOfWeekComponent<'a, Message> {
    value: ReminderDaysOfWeek,
    on_change: Option<Box<dyn Fn(ReminderDaysOfWeek) -> Message + 'a>>,
}

impl<'a, Message> DaysOfWeekComponent<'a, Message> {
    /// Sets the function to be called when the days of the week are changed.
    #[inline]
    pub fn on_change<F>(mut self, f: F) -> Self
    where
        F: Fn(ReminderDaysOfWeek) -> Message + 'a,
    {
        self.on_change = Some(Box::new(f));
        self
    }
}

impl<'a, Message> Component<Message, Renderer> for DaysOfWeekComponent<'a, Message> {
    type State = ();
    type Event = DaysOfWeekComponentEvent;

    fn update(&mut self, (): &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            DaysOfWeekComponentEvent::Set(days, enabled) => {
                self.value.set(days, enabled);
                self.on_change.as_ref().map(|f| f(self.value))
            }
        }
    }

    fn view(&self, (): &Self::State) -> Element<'_, Self::Event, Renderer> {
        const BUTTONS: &[(&str, ReminderDaysOfWeek)] = &[
            ("Sun", ReminderDaysOfWeek::SUNDAY),
            ("Mon", ReminderDaysOfWeek::MONDAY),
            ("Tue", ReminderDaysOfWeek::TUESDAY),
            ("Wed", ReminderDaysOfWeek::WEDNESDAY),
            ("Thu", ReminderDaysOfWeek::THURSDAY),
            ("Fri", ReminderDaysOfWeek::FRIDAY),
            ("Sat", ReminderDaysOfWeek::SATURDAY),
        ];

        let buttons = BUTTONS
            .iter()
            .copied()
            .map(|(label, days)| {
                let enabled = self.value.contains(days);
                button(
                    text(label)
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Center)
                        .vertical_alignment(Vertical::Center),
                )
                .on_press(DaysOfWeekComponentEvent::Set(days, !enabled))
                .width(Length::Fill)
                .style(if enabled {
                    Button::Secondary
                } else {
                    Button::Text
                })
                .into()
            })
            .collect();

        row(buttons).into()
    }
}

impl<'a, Message> From<DaysOfWeekComponent<'a, Message>> for Element<'a, Message, Renderer>
where
    Message: 'a,
{
    #[inline]
    fn from(value: DaysOfWeekComponent<'a, Message>) -> Self {
        component(value)
    }
}

/// An event for [`DaysOfWeekComponent`].
#[derive(Clone, Debug)]
pub enum DaysOfWeekComponentEvent {
    /// Sets whether a day is enabled.
    Set(ReminderDaysOfWeek, bool),
}
