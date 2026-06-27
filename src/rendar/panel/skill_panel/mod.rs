pub(crate) use crate::hero::data::SkillData;

/// スキル一覧を横並びで表示する。
pub(crate) fn skill_row(ui: &mut egui::Ui, skills: &[SkillData], current_level: u8) {
    ui.horizontal(|ui| {
        for (i, skill) in skills.iter().enumerate() {
            if i > 0 {
                ui.separator();
            }

            ui.vertical(|ui| {
                if ui.add(
                    egui::Button::image(
                        egui::Image::new(skill.image.clone()).fit_to_original_size(1.0),
                    ).frame(false),
                ).clicked() {
                    println!("{}: {}", skill.id, skill.name);
                }
                ui.label(format!("{}/{}", current_level, skill.max_level));
            });
        }
    });
}
