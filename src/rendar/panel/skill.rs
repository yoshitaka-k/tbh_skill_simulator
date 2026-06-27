use super::skill_panel::skill_row;
use crate::App;

/// スキルパネルを表示する。
pub(crate) fn skill_list_panel(ui: &mut egui::Ui, app: &mut App) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(10.0);

        for i in 0..app.hero().skill_list.len() {
            skill_row(ui, &mut app.hero_mut(), i);
            ui.separator();
        }

        ui.add_space(10.0);
    });
}
