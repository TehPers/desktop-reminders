use iced_lazy::{component, Component};
use iced_native::{
    widget::{column, scrollable},
    Element, Length,
};

use crate::{
    models::reminders::Reminder,
    ui::{app::Renderer, components::reminder},
};

/// A page that displays reminders for a specific day.
#[inline]
pub fn reminder_page<'a, Message>(reminders: &'a [Reminder]) -> ReminderPage<'a, Message> {
    ReminderPage {
        reminders,
        on_reminder_toggled: None,
    }
}

/// The state of a day page.
#[must_use]
pub struct ReminderPage<'a, Message> {
    reminders: &'a [Reminder],
    on_reminder_toggled: Option<Box<dyn Fn(usize, bool) -> Message + 'a>>,
}

impl<'a, Message> ReminderPage<'a, Message> {
    /// Sets the function to be called when a reminder is toggled.
    #[inline]
    pub fn on_toggle<F>(mut self, f: F) -> Self
    where
        F: Fn(usize, bool) -> Message + 'a,
    {
        self.on_reminder_toggled = Some(Box::new(f));
        self
    }
}

impl<'a, Message> Component<Message, Renderer> for ReminderPage<'a, Message> {
    type State = ();
    type Event = ReminderPageEvent;

    fn update(&mut self, (): &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            ReminderPageEvent::CompletedChanged(index, state) => {
                self.on_reminder_toggled.as_ref().map(|f| f(index, state))
            }
        }
    }

    fn view(&self, (): &Self::State) -> Element<Self::Event, Renderer> {
        let reminders = self
            .reminders
            .iter()
            .enumerate()
            .map(|(index, r)| {
                reminder(r)
                    .on_completed_changed(move |state| {
                        ReminderPageEvent::CompletedChanged(index, state)
                    })
                    .into()
            })
            .collect();

        scrollable(column(reminders)).width(Length::Fill).into()
    }
}

impl<'a, Message> From<ReminderPage<'a, Message>> for Element<'a, Message, Renderer>
where
    Message: 'a,
{
    #[inline]
    fn from(value: ReminderPage<'a, Message>) -> Self {
        component(value)
    }
}

/// An event for [`ReminderPage`].
pub enum ReminderPageEvent {
    /// A reminder's completed state was changed.
    CompletedChanged(usize, bool),
}
