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

            ui.vertical(|ui| {
                let button = ui.add(
                    egui::Button::image(
                        egui::Image::new(skill.image.clone()).fit_to_original_size(1.0),
                    ).frame(false),
                );

                if button.clicked() {
                    println!("{}: {}", skill.id, skill.name);
                    if hero.skill_points > 0 {
                        hero.skill_points -= 1;
                        skill.increase_level();
                    }
                } else if button.secondary_clicked() {
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
