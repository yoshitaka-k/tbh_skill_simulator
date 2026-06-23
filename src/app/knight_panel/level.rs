use crate::App;

/// レベルパネルを表示する。
pub(crate) fn level_panel(ui: &mut egui::Ui, app: &mut App) {
    ui.horizontal(|ui| {
        ui.label("Level:");
        if ui.add(egui::Slider::new(&mut app.level, 1..=100)).changed() {
            app.skill_points = app.level - 1;
        }

        ui.horizontal(|ui| {
            if ui.button("Down").clicked() {
                if app.level > 1 {
                    app.level -= 1;
                }
                if app.skill_points > 0 {
                    app.skill_points -= 1;
                }
            }
            if ui.button("Up").clicked() {
                app.level += 1;
                if app.skill_points < 100 {
                    app.skill_points += 1;
                }
            }
        });
    });

    ui.horizontal(|ui| {
        ui.label("Skill Points:");
        ui.label(app.skill_points.to_string());
    });
}
