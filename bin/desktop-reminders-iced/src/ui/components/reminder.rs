use iced_lazy::{component, Component};
use iced_native::{
    widget::{checkbox, container, horizontal_space, row, text},
    Element, Length, Padding,
};

use crate::{
    models::reminders::{Reminder, ReminderTimeOfDay},
    ui::app::Renderer,
};

/// Creates a [`ReminderComponent`].
#[inline]
pub fn reminder<'a, Message>(reminder: &'a Reminder) -> ReminderComponent<'a, Message> {
    ReminderComponent {
        reminder,
        on_completed_changed: None,
    }
}

/// A component for displaying a [`Reminder`].
#[must_use]
pub struct ReminderComponent<'a, Message> {
    reminder: &'a Reminder,
    on_completed_changed: Option<Box<dyn Fn(bool) -> Message + 'a>>,
}

impl<'a, Message> ReminderComponent<'a, Message> {
    /// Sets the function to be called when the completed state changes.
    #[inline]
    pub fn on_completed_changed<F>(mut self, f: F) -> Self
    where
        F: Fn(bool) -> Message + 'a,
    {
        self.on_completed_changed = Some(Box::new(f));
        self
    }
}

impl<'a, Message> Component<Message, Renderer> for ReminderComponent<'a, Message> {
    type State = ();
    type Event = ReminderComponentEvent;

    fn update(&mut self, (): &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            ReminderComponentEvent::CompletedChanged(state) => {
                self.on_completed_changed.as_ref().map(|f| f(state))
            }
        }
    }

    fn view(&self, (): &Self::State) -> iced_native::Element<'_, Self::Event, Renderer> {
        let cb = checkbox(
            &self.reminder.message,
            self.reminder.completed,
            ReminderComponentEvent::CompletedChanged,
        );
        let time = match self.reminder.frequency.time_of_day() {
            ReminderTimeOfDay::AllDay => text("All day").into(),
            ReminderTimeOfDay::Time { time } => text(time.format("%-I:%M %p")).into(),
            ReminderTimeOfDay::TimeRange { start, end } => text(format!(
                "{} - {}",
                start.format("%-I:%M %p"),
                end.format("%-I:%M %p")
            ))
            .into(),
        };

        container(row(vec![
            cb.into(),
            horizontal_space(Length::Fill).into(),
            time,
        ]))
        .padding(Padding {
            top: 5.0,
            right: 20.0,
            bottom: 5.0,
            left: 10.0,
        })
        .into()
    }
}

impl<'a, Message> From<ReminderComponent<'a, Message>> for Element<'a, Message, Renderer>
where
    Message: 'a,
{
    #[inline]
    fn from(value: ReminderComponent<'a, Message>) -> Self {
        component(value)
    }
}

/// The event of a [`ReminderComponent`].
pub enum ReminderComponentEvent {
    /// The completed state changed.
    CompletedChanged(bool),
}
