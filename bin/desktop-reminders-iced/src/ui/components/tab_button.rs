use iced_lazy::{component, Component};
use iced_native::{
    alignment::{Horizontal, Vertical},
    theme::Button,
    widget::{button, text},
    Element, Length,
};

use crate::ui::app::Renderer;

/// Creates a new [`TabButton`].
pub fn tab_button<'a, Message>(label: impl ToString) -> TabButton<'a, Message> {
    TabButton {
        label: label.to_string(),
        selected: false,
        on_select: None,
    }
}

/// A tab button.
pub struct TabButton<'a, Message> {
    label: String,
    selected: bool,
    on_select: Option<Box<dyn Fn() -> Message + 'a>>,
}

impl<'a, Message> TabButton<'a, Message> {
    /// Sets whether or not the tab is selected.
    #[inline]
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Sets the function to be called when the tab is selected.
    #[inline]
    pub fn on_select<F>(mut self, f: F) -> Self
    where
        F: Fn() -> Message + 'a,
    {
        self.on_select = Some(Box::new(f));
        self
    }
}

impl<'a, Message> Component<Message, Renderer> for TabButton<'a, Message> {
    type State = ();
    type Event = TabEvent;

    fn update(&mut self, (): &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            TabEvent::Selected => self.on_select.as_ref().map(|f| f()),
        }
    }

    fn view(&self, (): &Self::State) -> Element<'_, Self::Event, Renderer> {
        button(
            text(&self.label)
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Bottom),
        )
        .on_press(TabEvent::Selected)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(if self.selected {
            Button::Secondary
        } else {
            Button::Text
        })
        .into()
    }
}

impl<'a, Message> From<TabButton<'a, Message>> for Element<'a, Message, Renderer>
where
    Message: 'a,
{
    #[inline]
    fn from(value: TabButton<'a, Message>) -> Self {
        component(value)
    }
}

/// The events that can occur on a [`Tab`].
#[derive(Clone, Debug)]
pub enum TabEvent {
    Selected,
}
