use crate::app::knight_panel::skill_panel::{SKILL_IMAGES, skill_row};
use crate::App;

/// スキルパネルを表示する。
pub(crate) fn skill_list_panel(ui: &mut egui::Ui, _app: &mut App) {
    egui::ScrollArea::vertical()
    .show(ui, |ui| {
        ui.add_space(10.0);

        for i in 0..SKILL_IMAGES.len() {
            skill_row(ui, SKILL_IMAGES[i], 0);
            ui.separator();
        }

        ui.add_space(10.0);
    });
}
