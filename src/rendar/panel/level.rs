use crate::{App, capitalize};
use crate::hero::Hero;
use crate::hero::level_group::LevelGroup;
use crate::rendar::ResponseExt;

const MAX_LEVEL: u32 = 101;

enum LevelSound {
    LeftClick,
    RightClick,
    Hover,
}

/// レベルパネルを表示する。
pub(crate) fn level_panel(ui: &mut egui::Ui, app: &mut App) {
    ui.heading(capitalize(&app.hero().name()));
    ui.separator();

    let mut pending_sounds: Vec<LevelSound> = Vec::new();

    ui.horizontal(|ui| {
        ui.label("Level:");
        let mut level = *app.hero_mut().level();
        let slider = ui.add(egui::Slider::new(&mut level, 1..=MAX_LEVEL));
        if slider.changed() {
            app.hero_mut().set_level(level);
            update_skill_level(&mut app.hero_mut());
            update_skill_points(&mut app.hero_mut());
            app.hero_mut().update_active_skill();
        }

        if slider.just_hovered(ui) {
            pending_sounds.push(LevelSound::Hover);
        }

        ui.horizontal(|ui| {
            let down_button = ui.button("Down");
            let up_button = ui.button("Up");

            if down_button.just_hovered(ui) {
                pending_sounds.push(LevelSound::Hover);
            }

            if down_button.clicked() {
                pending_sounds.push(LevelSound::RightClick);

                if level > 1 {
                    app.hero_mut().decrease_level();
                }
                update_skill_level(&mut app.hero_mut());
                update_skill_points(&mut app.hero_mut());
                app.hero_mut().update_active_skill();
            }

            if up_button.just_hovered(ui) {
                pending_sounds.push(LevelSound::Hover);
            }

            if up_button.clicked() {
                pending_sounds.push(LevelSound::LeftClick);

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

    // 音声を再生する。
    for sound in pending_sounds {
        match sound {
            LevelSound::LeftClick => app.play_left_click_sound(),
            LevelSound::RightClick => app.play_right_click_sound(),
            LevelSound::Hover => app.play_hover_sound(),
        }
    }
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
