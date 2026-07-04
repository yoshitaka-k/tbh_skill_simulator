mod panel;

use egui::Color32;

use crate::App;
use panel::{detail_panel, effects_panel};

const DETAIL_HEIGHT: f32 = 120.0;
const COLOR_YELLOW: Color32 = Color32::from_rgb(200, 170, 80);

pub(crate) fn skill_detail_panel(ui: &mut egui::Ui, app: &mut App) {
    let w = ui.available_width();
    let total_h = ui.available_height();
    let sep = ui.spacing().item_spacing.y + 1.0;
    let bottom_h = (total_h - DETAIL_HEIGHT - sep).max(0.0);

    ui.allocate_ui(egui::vec2(w, DETAIL_HEIGHT), |ui| {
        ui.set_height(DETAIL_HEIGHT);
        egui::ScrollArea::vertical()
            .id_salt("skill_detail")
            .max_height(DETAIL_HEIGHT)
            .show(ui, |ui| {
                if let Some(hover_skill) = app.hover_skill() {
                    detail_panel(ui, hover_skill);
                } else if let Some(click_skill) = app.click_skill() {
                    detail_panel(ui, click_skill);
                }
            });
    });

    ui.separator();

    ui.allocate_ui(egui::vec2(w, bottom_h), |ui| {
        ui.set_height(bottom_h);
        egui::ScrollArea::vertical()
            .id_salt("skill_effects")
            .max_height(bottom_h)
            .show(ui, |ui| {
                effects_panel(ui, app.hero());
            });
    });
}
