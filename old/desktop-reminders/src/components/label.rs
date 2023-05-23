use egui::{Color32, FontSelection, Response, Sense, TextStyle, Ui, Widget, WidgetText};

#[derive(Clone)]
pub struct Label {
    text: WidgetText,
    color: Color32,
    style: TextStyle,
}

impl Default for Label {
    fn default() -> Self {
        Self {
            text: WidgetText::default(),
            color: Color32::default(),
            style: TextStyle::Body,
        }
    }
}

impl Label {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }
}

impl Widget for Label {
    fn ui(self, ui: &mut Ui) -> Response {
        let text = WidgetText::from(self.text);
        let job = text.into_text_job(
            ui.style(),
            FontSelection::Default,
            ui.layout().vertical_align(),
        );
        let galley = ui.ctx().fonts(|fonts| job.into_galley(fonts));
        let (id, rect) = ui.allocate_space(galley.size());
        let response = ui.interact(rect, id, Sense::hover());
        let text_color = self.color;
        let painter = ui.painter_at(rect);
        galley.paint_with_fallback_color(&painter, rect.left_center(), text_color);
        response
    }
}
