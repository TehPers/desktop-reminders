use std::sync::Arc;

use chrono::{NaiveDate, NaiveTime, Timelike};
use egui::{ComboBox, DragValue, Ui};
use egui_extras::DatePickerButton;

use super::{Configurable, IdTree};

impl Configurable for NaiveDate {
    type Options = ();

    fn config_ui(&mut self, id: Arc<IdTree>, ui: &mut Ui, _options: Self::Options) {
        ui.add(DatePickerButton::new(self).id_source(&id.child("date").to_string()));
    }
}

impl Configurable for NaiveTime {
    type Options = ();

    fn config_ui(&mut self, id: Arc<IdTree>, ui: &mut Ui, _options: Self::Options) {
        let (mut pm, mut hour) = self.hour12();
        let mut minute = self.minute();

        ui.add(
            DragValue::new(&mut hour)
                .speed(1.0)
                .clamp_range(1..=12)
                .custom_formatter(|n, _| format!("{:02}", n)),
        );
        ui.label(":");
        ui.add(
            DragValue::new(&mut minute)
                .speed(1.0)
                .clamp_range(0..=59)
                .custom_formatter(|n, _| format!("{:02}", n)),
        );
        ComboBox::from_id_source(id.child("am_pm"))
            .selected_text(if pm { "PM" } else { "AM" })
            .show_ui(ui, |ui| {
                let am_label = ui.selectable_label(!pm, "AM");
                let pm_label = ui.selectable_label(pm, "PM");

                if am_label.clicked() {
                    pm = false;
                } else if pm_label.clicked() {
                    pm = true;
                }
            });

        hour %= 12;
        if pm {
            hour += 12;
        }

        *self = NaiveTime::from_hms_opt(hour, minute, 0).unwrap_or(*self);
    }
}
