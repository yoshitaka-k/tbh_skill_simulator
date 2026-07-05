pub(crate) mod detail;

pub(crate) use crate::hero::Skill;
use crate::rendar::panel::{COLOR_YELLOW, COLOR_GRAY, HOVER_COLOR_YELLOW, HOVER_COLOR_GRAY, HOVER_COLOR_GRAY_NOT_ACTIVE};
use crate::app::App;
use crate::hero::level_group::LevelGroup;

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
    let skills = &hero.skill_list_by_group(group);
    let mut pending_changes: Vec<SkillChange> = Vec::new();
    let mut pending_details: Vec<SkillDetail> = Vec::new();

    ui.horizontal(|ui| {
        for (i, skill) in skills.iter().enumerate() {
            let tint_color = tint_color(*skill.active());
            let image = skill_image(skill, tint_color);

            if i > 0 {
                ui.separator();
            }

            ui.vertical(|ui| {
                let button = ui.add(egui::Button::image(image).frame(false));
                let border_color = if button.hovered() {
                    hover_border_color(*skill.level(), *skill.active())
                } else {
                    border_color(*skill.level())
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
                    if *skill.active() {
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
                    pending_changes.push(SkillChange::Decrease {
                        group: *group,
                        index: i,
                    });

                    pending_details.push(SkillDetail::Click {
                        group: *group,
                        index: i,
                    });
                }

                if skill.is_max_level() {
                    ui.colored_label(COLOR_YELLOW, format!("{}/{}", skill.level(), skill.max_level()));
                } else {
                    ui.label(format!("{}/{}", skill.level(), skill.max_level()));
                }
            });
        }
    });

    // スキルレベルの変更を適用する。
    for change in pending_changes {
        match change {
            SkillChange::Increase { group, index } => {
                // スキルポイントがあり、スキルレベルが最大レベル未満の場合は、スキルレベルを増やす。
                if *hero.skill_points() > 0 && hero.skill_level(&group, index) < hero.skill_max_level(&group, index) {
                    hero.decrease_skill_points();
                    hero.increase_skill_level(&group, index);
                }
            },
            SkillChange::Decrease { group, index } => {
                // スキルレベルが0より大きい場合は、スキルレベルを減らす。
                if hero.skill_level(&group, index) > 0 {
                    if hero.decrease_skill_level(&group, index) {
                        hero.increase_skill_points();
                    }
                }
            }
        }

        hero.update_active_skill();
    }

    // スキル詳細の表示を更新する。
    for detail in pending_details {
        match detail {
            SkillDetail::Hover { group, index } => {
                if let Some(skill) = app.hero().skill(&group, index).cloned() {
                    app.set_hover_skill(Some(skill));
                }
            },
            SkillDetail::Click { group, index } => {
                if let Some(skill) = app.hero().skill(&group, index).cloned() {
                    app.set_click_skill(Some(skill));
                }
            }
        }
    }
}

/// スキル画像を作成する。
fn skill_image(skill: &Skill, tint_color: egui::Color32) -> egui::Image<'_> {
    egui::Image::new(skill.image().clone())
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
        COLOR_YELLOW
    } else {
        COLOR_GRAY
    }
}

/// スキルレベルに応じて hover border 色を返す。
fn hover_border_color(level: u32, active: bool) -> egui::Color32 {
    if active{
        if level > 0 {
            HOVER_COLOR_YELLOW
        } else {
            HOVER_COLOR_GRAY
        }
    } else {
        HOVER_COLOR_GRAY_NOT_ACTIVE
    }
}
