pub(crate) mod detail;

pub(crate) use crate::hero::Skill;
use crate::app::App;
use crate::app::level_group::LevelGroup;

enum SkillChange {
    Increase { group: LevelGroup, index: usize },
    Decrease { group: LevelGroup, index: usize },
}

enum SkillDetail {
    Hover { group: LevelGroup, index: usize },
    Click { group: LevelGroup, index: usize },
}

/// スキル一覧を横並びで表示する。
pub(crate) fn skill_row(ui: &mut egui::Ui, app: &mut App, group: &LevelGroup) {
    let hero = app.hero_mut();
    let skills = &hero.skill_list[group];
    let mut pending_changes: Vec<SkillChange> = Vec::new();
    let mut pending_details: Vec<SkillDetail> = Vec::new();

    ui.horizontal(|ui| {
        for (i, skill) in skills.iter().enumerate() {
            let tint_color = tint_color(skill.active);
            let image = skill_image(skill, tint_color);

            if i > 0 {
                ui.separator();
            }

            ui.vertical(|ui| {
                let button = ui.add(egui::Button::image(image).frame(false));
                let border_color = if button.hovered() {
                    hover_border_color(skill.level, skill.active)
                } else {
                    border_color(skill.level)
                };

                // Button の frame/stroke はホバー時に inner_margin が変わりサイズがずれるため、
                // painter で枠線を描画する。
                ui.painter().rect_stroke(
                    button.rect,
                    0.0,
                    egui::Stroke::new(1.0, border_color),
                    egui::StrokeKind::Inside,
                );

                if button.hovered() {
                    pending_details.push(SkillDetail::Hover {
                        group: *group,
                        index: i,
                    });
                }

                if button.clicked() {
                    if skill.active {
                        println!("{}: {} increase", skill.id, skill.name);
                        pending_changes.push(SkillChange::Increase {
                            group: *group,
                            index: i,
                        });
                    }
                    pending_details.push(SkillDetail::Click {
                        group: *group,
                        index: i,
                    });

                } else if button.secondary_clicked() {
                    if skill.active {
                        println!("{}: {} decrease", skill.id, skill.name);
                        pending_changes.push(SkillChange::Decrease {
                            group: *group,
                            index: i,
                        });
                    }
                    pending_details.push(SkillDetail::Click {
                        group: *group,
                        index: i,
                    });
                }

                ui.label(format!("{}/{}", skill.level, skill.max_level()));
            });
        }
    });

    for change in pending_changes {
        match change {
            SkillChange::Increase { group, index } => {
                if hero.skill_points > 0 && hero.skill_level(&group, index) < hero.skill_max_level(&group, index) {
                    hero.skill_points -= 1;
                    hero.increase_skill_level(&group, index);
                }
            },
            SkillChange::Decrease { group, index } => {
                if hero.skill_level(&group, index) > 0 {
                    hero.skill_points += 1;
                    hero.decrease_skill_level(&group, index);
                }
            }
        }

        hero.update_active_skill();
    }

    for detail in pending_details {
        match detail {
            SkillDetail::Hover { group, index } => {
                if let Some(skill) = &app.hero().skill(&group, index) {
                    app.set_hover_skill_detail(Some(format!(
                        "{}\n{}\n\n{}\n\n{}",
                        skill.skill_type.clone(),
                        skill.name.clone(),
                        skill.description_display(),
                        skill.effects_display()
                    )));
                }
            },
            SkillDetail::Click { group, index } => {
                if let Some(skill) = &app.hero().skill(&group, index) {
                    app.set_click_skill_detail(Some(format!(
                        "{}\n{}\n\n{}\n\n{}",
                        skill.skill_type.clone(),
                        skill.name.clone(),
                        skill.description_display(),
                        skill.effects_display()
                    )));
                }
            }
        }
    }
}

/// スキル画像を作成する。
fn skill_image(skill: &Skill, tint_color: egui::Color32) -> egui::Image<'_> {
    egui::Image::new(skill.image.clone())
        .fit_to_original_size(1.0)
        .tint(tint_color)
}

/// スキルレベルに応じて tint 色を返す。
fn tint_color(active: bool) -> egui::Color32 {
    if active {
        egui::Color32::WHITE
    } else {
        egui::Color32::from_gray(100)
    }
}

/// スキルレベルに応じて border 色を返す。
fn border_color(level: u32) -> egui::Color32 {
    if level > 0 {
        egui::Color32::from_rgb(200, 170, 80)
    } else {
        egui::Color32::from_gray(50)
    }
}

/// スキルレベルに応じて hover border 色を返す。
fn hover_border_color(level: u32, active: bool) -> egui::Color32 {
    if active{
        if level > 0 {
            egui::Color32::from_rgb(255, 220, 100)
        } else {
            egui::Color32::from_gray(120)
        }
    } else {
        egui::Color32::from_gray(50)
    }
}
