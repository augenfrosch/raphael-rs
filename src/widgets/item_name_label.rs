use raphael_data::{Locale, get_item_name};

pub struct ItemNameLabel {
    item_id: u32,
    text: String,
}

impl ItemNameLabel {
    pub fn new(item_id: u32, hq: bool, locale: Locale) -> Self {
        Self {
            item_id,
            text: get_item_name(item_id, hq, locale).unwrap_or("Unknown item".to_owned()),
        }
    }
}

impl egui::Widget for ItemNameLabel {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let id = ui.id().with(self.item_id);

        let response = if ui.ctx().animate_bool_with_time(id, false, 0.25) == 0.0 {
            ui.add(egui::Label::new(egui::RichText::new(&self.text)).sense(egui::Sense::CLICK))
        } else {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(&self.text).color(ui.style().visuals.weak_text_color()),
                )
                .sense(egui::Sense::click()),
            )
        };

        response.context_menu(|ui| {
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                ui.close();
            }
            if ui.button("Copy item name").clicked() {
                let copy_item_name = self
                    .text
                    .trim_end_matches([' ', raphael_data::HQ_ICON_CHAR, raphael_data::CL_ICON_CHAR])
                    .to_string();
                ui.ctx().copy_text(copy_item_name);
                ui.close();
                ui.ctx().animate_bool_with_time(id, true, 0.0);
            }
        });
        response
    }
}
