use super::skill_panel::skill_row;
use crate::App;

/// スキルパネルを表示する。
pub(crate) fn skill_list_panel(ui: &mut egui::Ui, app: &mut App) {
    app.hero_mut().skill_detail.clear();

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(10.0);

        let hero = app.hero_mut();
        let groups: Vec<_> = hero.skill_list.keys().copied().collect();

        for group in groups {
            skill_row(ui, hero, &group);
            ui.separator();
        }

        ui.add_space(10.0);
    });
}
