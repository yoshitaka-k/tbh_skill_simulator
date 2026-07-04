use crate::{App, capitalize};
use crate::hero::Hero;
use crate::hero::level_group::LevelGroup;

const MAX_LEVEL: u32 = 101;

/// レベルパネルを表示する。
pub(crate) fn level_panel(ui: &mut egui::Ui, app: &mut App) {
    ui.heading(capitalize(&app.hero().name()));
    ui.separator();

    ui.horizontal(|ui| {
        ui.label("Level:");
        let mut level = *app.hero_mut().level();
        if ui.add(egui::Slider::new(&mut level, 1..=MAX_LEVEL)).changed() {
            app.hero_mut().set_level(level);
            update_skill_level(&mut app.hero_mut());
            update_skill_points(&mut app.hero_mut());
            app.hero_mut().update_active_skill();
        }

        ui.horizontal(|ui| {
            if ui.button("Down").clicked() {
                if level > 1 {
                    app.hero_mut().decrease_level();
                }
                update_skill_level(&mut app.hero_mut());
                update_skill_points(&mut app.hero_mut());
                app.hero_mut().update_active_skill();
            }
            if ui.button("Up").clicked() {
                app.hero_mut().increase_level();
                update_skill_level(&mut app.hero_mut());
                update_skill_points(&mut app.hero_mut());
                app.hero_mut().update_active_skill();
            }
        });
    });

    ui.horizontal(|ui| {
        ui.label("Skill Points:");
        ui.label(app.hero().skill_points().to_string());
    });
}

/// スキルポイントを更新する。
fn update_skill_points(hero: &mut Hero) {
    if *hero.level() as i32 - hero.skill_level_sum() as i32 > 0 {
        hero.set_skill_points(hero.level() - hero.skill_level_sum());
    } else {
        hero.set_skill_points(0);
    }
}

/// スキルレベルを更新する。
fn update_skill_level(hero: &mut Hero) {
    let mut level_group = LevelGroup::Level70;
    while *hero.level() < hero.skill_level_sum() {
        let skill_list = hero.skill_list_by_group_mut(&level_group);

        for skill in skill_list.iter_mut() {
            while *skill.level() > 0 {
                skill.decrease_level();
            }
        }

        if let Some(prev_level_group) = level_group.prev() {
            level_group = prev_level_group;
        } else {
            break;
        }
    }
}
