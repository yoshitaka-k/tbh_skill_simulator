pub(crate) use crate::hero::data::HeroData;

use crate::rendar::ResponseExt;
use crate::app::current_hero::CurrentHero;
use crate::App;
use crate::rendar::panel::{COLOR_YELLOW, COLOR_GRAY, HOVER_COLOR_YELLOW, HOVER_COLOR_GRAY, HERO_IMAGE_GRAY_SCALE};

enum HeroSound {
    LeftClick,
    Hover,
}

/// キャラクターパネルを表示する。
pub(crate) fn hero_row(ui: &mut egui::Ui, hero: &HeroData, app: &mut App) {
    let mut pending_sounds: Vec<HeroSound> = Vec::new();

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

        // ホバー開始時に1回だけ音声を再生する。
        if button.just_hovered(ui) {
            pending_sounds.push(HeroSound::Hover);
        }

        if button.clicked() {
            pending_sounds.push(HeroSound::LeftClick);

            app.set_current_hero(match hero.name {
                "knight" => CurrentHero::Knight,
                "ranger" => CurrentHero::Ranger,
                "slayer" => CurrentHero::Slayer,
                "sorcerer" => CurrentHero::Sorcerer,
                "priest" => CurrentHero::Priest,
                "hunter" => CurrentHero::Hunter,
                _ => CurrentHero::Knight,
            });

            app.set_hover_skill(None);
            app.set_click_skill(None);
        }
    });

    // 音声を再生する。
    for sound in pending_sounds {
        match sound {
            HeroSound::LeftClick => app.play_left_click_sound(),
            HeroSound::Hover => app.play_hover_sound(),
        }
    }
}

/// キャラクター画像を作成する。
fn hero_image(hero: &HeroData, tint_color: egui::Color32) -> egui::Image<'_> {
    egui::Image::new(hero.image.clone())
        .fit_to_original_size(1.0)
        .tint(tint_color)
}

/// キャラクター tint 色を返す。
fn tint_color(hero: &HeroData, app: &App) -> egui::Color32 {
    if hero.name == app.current_hero().to_string() {
        egui::Color32::WHITE
    } else {
        HERO_IMAGE_GRAY_SCALE
    }
}

/// キャラクター border 色を返す。
fn border_color(hero: &HeroData, app: &App) -> egui::Color32 {
    if hero.name == app.current_hero().to_string() {
        COLOR_YELLOW
    } else {
        COLOR_GRAY
    }
}

/// キャラクター hover border 色を返す。
fn hover_border_color(hero: &HeroData, app: &App) -> egui::Color32 {
    if hero.name == app.current_hero().to_string() {
        HOVER_COLOR_YELLOW
    } else {
        HOVER_COLOR_GRAY
    }
}
