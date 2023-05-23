use egui::{text::LayoutJob, Color32, FontId, Response, Style, TextFormat, TextStyle, Ui, Widget};

use crate::models::reminder::Reminder;

/// A view of a reminder.
#[derive(Debug)]
pub struct ReminderView<'a> {
    reminder: &'a mut Reminder,
}

impl<'a> ReminderView<'a> {
    pub const DATE_STYLE_NAME: &str = "reminder_view::date";
    pub const MESSAGE_STYLE_NAME: &str = "reminder_view::message";

    /// Create a new reminder view.
    #[inline]
    #[must_use]
    pub fn new(reminder: &'a mut Reminder) -> Self {
        Self { reminder }
    }
}

impl<'a> Widget for ReminderView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let completed = self.reminder.completed;
        ui.horizontal(|ui| {
            if completed {
                ui.style_mut().visuals.widgets.noninteractive.bg_fill = Color32::LIGHT_GREEN;
            }

            let mut contents = LayoutJob::default();
            let style = ui.style();

            // Get font IDs
            let date_font_id = get_font_id(
                style,
                &[
                    TextStyle::Name(Self::DATE_STYLE_NAME.into()),
                    TextStyle::Body,
                ],
            );
            let message_font_id = get_font_id(
                style,
                &[
                    TextStyle::Name(Self::MESSAGE_STYLE_NAME.into()),
                    TextStyle::Body,
                ],
            );

            // Date
            contents.append(
                &"TODO",
                0.0,
                TextFormat {
                    font_id: date_font_id.clone(),
                    color: style.visuals.strong_text_color(),
                    ..Default::default()
                },
            );

            // Separator
            contents.append(
                " - ",
                0.0,
                TextFormat {
                    font_id: date_font_id,
                    color: style.visuals.strong_text_color(),
                    ..Default::default()
                },
            );

            // Message
            contents.append(
                &self.reminder.message,
                0.0,
                TextFormat {
                    font_id: message_font_id,
                    color: style.visuals.text_color(),
                    ..Default::default()
                },
            );

            ui.checkbox(&mut self.reminder.completed, contents);
        })
        .response
    }
}

fn get_font_id(style: &Style, text_styles: &[TextStyle]) -> FontId {
    // Prepend the override text style if it exists
    let prepend_styles = if let Some(text_style) = style.override_text_style.clone() {
        vec![text_style]
    } else {
        vec![]
    };
    let mut text_style_iter = prepend_styles.iter().chain(text_styles);

    // Get the font ID
    style
        .override_font_id
        .clone()
        .or_else(|| {
            text_style_iter
                .find_map(|text_style| style.text_styles.get(text_style))
                .cloned()
        })
        .unwrap_or_default()
}
