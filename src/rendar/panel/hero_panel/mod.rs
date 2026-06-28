pub(crate) use crate::hero::data::HeroData;

use crate::{app::CurrentHero, App};

/// キャラクターパネルを表示する。
pub(crate) fn hero_row(ui: &mut egui::Ui, hero: &HeroData, app: &mut App) {
    ui.vertical(|ui| {
        let tint_color = tint_color(hero, app);
        let image = hero_image(hero, tint_color);

        let button = ui.add(egui::Button::image(image).frame(false));
        let border_color = if button.hovered() {
            hover_border_color(hero, app)
        } else {
            border_color(hero, app)
        };

        // 枠線を描画する。
        ui.painter().rect_stroke(
            button.rect,
            0.0,
            egui::Stroke::new(1.0, border_color),
            egui::StrokeKind::Inside,
        );

        if button.clicked() {
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

/// キャラクター画像を作成する。
fn hero_image(hero: &HeroData, tint_color: egui::Color32) -> egui::Image<'_> {
    egui::Image::new(hero.image.clone())
        .fit_to_original_size(1.0)
        .tint(tint_color)
}

/// キャラクター tint 色を返す。
fn tint_color(hero: &HeroData, app: &App) -> egui::Color32 {
    if hero.name == app.current_hero.to_string() {
        egui::Color32::WHITE
    } else {
        egui::Color32::from_gray(100)
    }
}

/// キャラクター border 色を返す。
fn border_color(hero: &HeroData, app: &App) -> egui::Color32 {
    if hero.name == app.current_hero.to_string() {
        egui::Color32::from_rgb(200, 170, 80)
    } else {
        egui::Color32::from_gray(50)
    }
}

/// キャラクター hover border 色を返す。
fn hover_border_color(hero: &HeroData, app: &App) -> egui::Color32 {
    if hero.name == app.current_hero.to_string() {
        egui::Color32::from_rgb(255, 220, 100)
    } else {
        egui::Color32::from_gray(100)
    }
}
