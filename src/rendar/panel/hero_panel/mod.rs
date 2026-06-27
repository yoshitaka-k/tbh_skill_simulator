pub(crate) use crate::hero::data::HeroData;

use crate::{app::CurrentHero, App};

/// キャラクターパネルを表示する。
pub(crate) fn hero_row(ui: &mut egui::Ui, hero: &HeroData, app: &mut App) {
    ui.vertical(|ui| {
        let tint = if hero.name == app.current_hero.to_string() {
            egui::Color32::WHITE
        } else {
            egui::Color32::from_gray(100)
        };

        let image = egui::Image::new(hero.image.clone()).fit_to_original_size(1.0).tint(tint);

        if ui.add(egui::Button::image(image).frame(false)).clicked() {
            println!("{}", hero.name);
            app.current_hero = match hero.name {
                "knight" => CurrentHero::Knight,
                "ranger" => CurrentHero::Ranger,
                "slayer" => CurrentHero::Slayer,
                "sorcerer" => CurrentHero::Sorcerer,
                "priest" => CurrentHero::Priest,
                "hunter" => CurrentHero::Hunter,
                _ => CurrentHero::Knight,
            };
        }
    });
}
