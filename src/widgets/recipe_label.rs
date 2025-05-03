use raphael_data::{CrafterStats, Locale, Recipe, get_game_settings, get_item_name};

// The game seems to not have the expert recipe icon as a text char, instead using a texture/image.
// We need to store it somewhere where it doesn't collide.
// The game uses up to ~`\u{e0e0}`, egui's default fonts use space from `\u{e600}` upwards.
pub const EXPERT_RECIPE_ICON_CHAR: char = '\u{e100}';
// The reason they use a texture is most likely since the icon typically appears on a baked in background
// or with a reddish glow behind it. egui doesn't seem to have an easy way of replicating this procedurally  
pub const EXPERT_RECIPE_ICON_COLOR: egui::Color32 = egui::Color32::from_rgb(224, 154, 122);

pub struct RecipeLabel<'a> {
    recipe: &'a Recipe,
    crafter_level: u8,
    locale: Locale,
}

impl<'a> RecipeLabel<'a> {
    pub fn new(recipe: &'a Recipe, crafter_level: u8, locale: Locale) -> Self {
        Self {
            recipe,
            crafter_level,
            locale,
        }
    }
}

impl<'a> egui::Widget for RecipeLabel<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let id = ui.id().with(self.recipe.item_id);
        let mut layout_job = egui::text::LayoutJob::default();
        let style = ui.style();

        let item_name = get_item_name(self.recipe.item_id, false, self.locale)
            .unwrap_or("Unknown item".to_owned());
        let item_name_text = if ui.ctx().animate_bool_with_time(id, false, 0.25) == 0.0 {
            egui::RichText::new(&item_name)
        } else {
            egui::RichText::new(&item_name).color(style.visuals.weak_text_color())
        };
        item_name_text.append_to(
            &mut layout_job,
            &style,
            egui::FontSelection::Default,
            egui::Align::Center,
        );

        if self.recipe.is_expert {
            egui::RichText::new(format!(" {}", EXPERT_RECIPE_ICON_CHAR))
                .color(EXPERT_RECIPE_ICON_COLOR)
                .append_to(
                    &mut layout_job,
                    &style,
                    egui::FontSelection::Default,
                    egui::Align::Center,
                );
        }
        if self.recipe.max_level_scaling != 0 {
            let game_settings = get_game_settings(
                *self.recipe,
                None,
                CrafterStats {
                    level: self.crafter_level,
                    ..Default::default()
                },
                None,
                None,
            );

            egui::RichText::new(format!(" {} ", raphael_data::LEVEL_SYNCED_ICON_CHAR))
                .color(style.visuals.widgets.inactive.fg_stroke.color)
                .append_to(
                    &mut layout_job,
                    &style,
                    egui::FontSelection::Default,
                    egui::Align::Center,
                );

            egui::RichText::new(format!("{: <5}", game_settings.max_progress))
                .color(style.visuals.widgets.inactive.fg_stroke.color)
                .size(7.0)
                .append_to(
                    &mut layout_job,
                    &style,
                    egui::FontSelection::Default,
                    egui::Align::TOP,
                );
            egui::RichText::new(format!("{: <5}", game_settings.max_quality))
                .color(style.visuals.widgets.inactive.fg_stroke.color)
                .size(7.0)
                .append_to(
                    &mut layout_job,
                    &style,
                    egui::FontSelection::Default,
                    egui::Align::BOTTOM,
                );
            layout_job.sections.last_mut().unwrap().leading_space = -20.0;
        }

        let response = ui.add(egui::Label::new(layout_job).sense(egui::Sense::CLICK));

        response.context_menu(|ui| {
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                ui.close();
            }
            let mut selection_made = false;
            if ui.button("Copy item name").clicked() {
                let copy_item_name = item_name
                    .trim_end_matches([' ', raphael_data::HQ_ICON_CHAR, raphael_data::CL_ICON_CHAR])
                    .to_string();
                ui.ctx().copy_text(copy_item_name);
                ui.close();
                selection_made = true;
            }

            if selection_made {
                ui.ctx().animate_bool_with_time(id, true, 0.0);
            }
        });
        response
    }
}
