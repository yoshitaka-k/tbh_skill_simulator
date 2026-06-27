pub(crate) use crate::hero::Skill;
use crate::hero::Hero;

/// スキル一覧を横並びで表示する。
pub(crate) fn skill_row(ui: &mut egui::Ui, hero: &mut Hero, index: usize) {
    let skills: &mut Vec<Skill> = &mut hero.skill_list[index];

    ui.horizontal(|ui| {
        for (i, skill) in skills.into_iter().enumerate() {
            if i > 0 {
                ui.separator();
            }

            let tint = if skill.level > 0 {
                egui::Color32::WHITE
            } else {
                egui::Color32::from_gray(100)
            };

            let border_color = if skill.level > 0 {
                egui::Color32::from_rgb(200, 170, 80)
            } else {
                egui::Color32::from_gray(50)
            };
            let hover_border_color = if skill.level > 0 {
                egui::Color32::from_rgb(255, 220, 100)
            } else {
                egui::Color32::from_gray(100)
            };

            let image = egui::Image::new(skill.image.clone())
                .fit_to_original_size(1.0)
                .tint(tint);

            ui.vertical(|ui| {
                // Button の frame/stroke はホバー時に inner_margin が変わりサイズがずれるため、
                // 枠線は painter で描画する。
                let button = ui.add(egui::Button::image(image).frame(false));

                let border_color = if button.hovered() {
                    hover_border_color
                } else {
                    border_color
                };

                ui.painter().rect_stroke(
                    button.rect,
                    0.0,
                    egui::Stroke::new(1.0, border_color),
                    egui::StrokeKind::Inside,
                );

                if button.clicked() {
                    println!("{}: {} increase", skill.id, skill.name);
                    if hero.skill_points > 0 {
                        hero.skill_points -= 1;
                        skill.increase_level();
                    }
                } else if button.secondary_clicked() {
                    println!("{}: {} decrease", skill.id, skill.name);
                    if hero.skill_points < hero.level {
                        hero.skill_points += 1;
                        skill.decrease_level();
                    }
                }

                ui.label(format!("{}/{}", skill.level, skill.max_level));
            });
        }
    });
}
