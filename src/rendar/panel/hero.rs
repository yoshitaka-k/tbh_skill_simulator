use crate::hero::data::heros::HERO_DATA;
use super::hero_panel::hero_row;
use crate::App;

/// キャラクターパネルを表示する。
pub(crate) fn hero_list_panel(ui: &mut egui::Ui, app: &mut App) {
    ui.horizontal(|ui| {
        for i in 0..HERO_DATA.len() {
            hero_row(ui, &HERO_DATA[i], app);
        }
    });
}
