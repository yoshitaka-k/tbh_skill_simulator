use crate::hero::data::{knight, ranger};
use super::skill_panel::skill_row;
use crate::{App, CurrentHero};

/// スキルパネルを表示する。
pub(crate) fn skill_list_panel(ui: &mut egui::Ui, app: &mut App) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(10.0);

        match app.current_hero {
            CurrentHero::Knight => {
                for i in 0..knight::SKILL_DATA.len() {
                    skill_row(ui, knight::SKILL_DATA[i], 0);
                    ui.separator();
                }
            }
            CurrentHero::Ranger => {
                for i in 0..ranger::SKILL_DATA.len() {
                    skill_row(ui, ranger::SKILL_DATA[i], 0);
                    ui.separator();
                }
            }
            _ => {
                for i in 0..knight::SKILL_DATA.len() {
                    skill_row(ui, knight::SKILL_DATA[i], 0);
                    ui.separator();
                }
            }
        }

        ui.add_space(10.0);
    });
}
