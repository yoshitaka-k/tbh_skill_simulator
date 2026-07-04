use crate::{App, capitalize, MAX_LEVEL};
use crate::hero::Hero;

/// レベルパネルを表示する。
pub(crate) fn level_panel(ui: &mut egui::Ui, app: &mut App) {
    ui.heading(capitalize(&app.hero().name()));
    ui.separator();

    ui.horizontal(|ui| {
        ui.label("Level:");
        let mut level = *app.hero_mut().level();
        if ui.add(egui::Slider::new(&mut level, 1..=MAX_LEVEL)).changed() {
            app.hero_mut().set_level(level);
            update_skill_points(&mut app.hero_mut());
        }

        ui.horizontal(|ui| {
            if ui.button("Down").clicked() {
                if level > 1 {
                    app.hero_mut().decrease_level();
                }
                update_skill_points(&mut app.hero_mut());
            }
            if ui.button("Up").clicked() {
                app.hero_mut().increase_level();
                update_skill_points(&mut app.hero_mut());
            }
        });
    });

    ui.horizontal(|ui| {
        ui.label("Skill Points:");
        ui.label(app.hero().skill_points().to_string());
    });

    /// スキルポイントを更新する。
    fn update_skill_points(hero: &mut Hero) {
        if *hero.level() as i32 - hero.skill_level_sum() as i32 > 0 {
            hero.set_skill_points(hero.level() - hero.skill_level_sum());
        } else {
            hero.set_skill_points(0);
        }
    }
}
