use iced_lazy::{component, Component};
use iced_native::{
    alignment::Horizontal,
    widget::{column, container, horizontal_rule, row},
    Element, Length,
};

use crate::ui::app::Renderer;

use super::tab_button;

/// Creates a [`TabList`].
pub fn tab_list<'a, K, Message>(tabs: impl IntoIterator<Item = Tab<K>>) -> TabList<'a, K, Message> {
    TabList {
        tabs: tabs.into_iter().collect(),
        on_tab_selected: None,
    }
}

/// A list of tabs.
#[must_use]
pub struct TabList<'a, K, Message> {
    tabs: Vec<Tab<K>>,
    on_tab_selected: Option<Box<dyn Fn(K) -> Message + 'a>>,
}

impl<'a, K, Message> TabList<'a, K, Message> {
    /// Sets the function to be called when a tab is selected.
    #[inline]
    pub fn on_selected<F>(mut self, f: F) -> Self
    where
        F: Fn(K) -> Message + 'a,
    {
        self.on_tab_selected = Some(Box::new(f));
        self
    }
}

impl<'a, K, Message> Component<Message, Renderer> for TabList<'a, K, Message>
where
    K: Clone,
{
    type State = ();
    type Event = TabListEvent<K>;

    fn update(&mut self, (): &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            TabListEvent::TabSelected(id) => self.on_tab_selected.as_ref().map(move |f| f(id)),
        }
    }

    fn view(&self, (): &Self::State) -> Element<'_, Self::Event, Renderer> {
        let tabs = self
            .tabs
            .iter()
            .map(|tab| {
                container(
                    tab_button(&tab.label)
                        .selected(tab.selected)
                        .on_select(|| TabListEvent::TabSelected(tab.id.clone())),
                )
                .width(Length::Fill)
                .align_x(Horizontal::Center)
                .into()
            })
            .collect();

        column(vec![row(tabs).into(), horizontal_rule(3).into()]).into()
    }
}

impl<'a, K, Message> From<TabList<'a, K, Message>> for Element<'a, Message, Renderer>
where
    K: Clone + 'a,
    Message: 'a,
{
    #[inline]
    fn from(value: TabList<'a, K, Message>) -> Self {
        component(value)
    }
}

/// The events of a [`TabList`].
#[derive(Clone, Debug)]
pub enum TabListEvent<K> {
    /// A tab was selected.
    TabSelected(K),
}

/// A tab in a [`TabList`].
#[must_use]
pub struct Tab<K> {
    /// The label.
    pub label: String,
    /// The ID.
    pub id: K,
    /// Whether or not the tab is selected.
    pub selected: bool,
}

impl<K> Tab<K> {
    /// Creates a new [`Tab`].
    #[inline]
    pub fn new(label: impl ToString, id: K, selected: bool) -> Self {
        Self {
            label: label.to_string(),
            id,
            selected,
        }
    }
}
