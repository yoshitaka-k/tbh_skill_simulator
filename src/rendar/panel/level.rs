use crate::{App, capitalize};

/// レベルパネルを表示する。
pub(crate) fn level_panel(ui: &mut egui::Ui, app: &mut App) {
    ui.heading(capitalize(&app.hero().name));
    ui.separator();

    ui.horizontal(|ui| {
        ui.label("Level:");
        if ui.add(egui::Slider::new(&mut app.hero_mut().level, 1..=100)).changed() {
            app.hero_mut().skill_points = app.hero_mut().level;
        }

        ui.horizontal(|ui| {
            if ui.button("Down").clicked() {
                if app.hero_mut().level > 1 {
                    app.hero_mut().level -= 1;
                }
                if app.hero_mut().skill_points > 0 {
                    app.hero_mut().skill_points -= 1;
                }
            }
            if ui.button("Up").clicked() {
                app.hero_mut().level += 1;
                if app.hero_mut().skill_points < 100 {
                    app.hero_mut().skill_points += 1;
                }
            }
        });
    });

    ui.horizontal(|ui| {
        ui.label("Skill Points:");
        ui.label(app.hero_mut().skill_points.to_string());
    });
}
