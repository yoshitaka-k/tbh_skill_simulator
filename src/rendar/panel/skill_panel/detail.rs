use egui::{Align, Color32, FontSelection, RichText, text::LayoutJob};

use crate::App;
use crate::hero::Skill;

const DETAIL_HEIGHT: f32 = 140.0;
const COLOR_YELLOW: Color32 = Color32::from_rgb(200, 170, 80);

pub(crate) fn skill_detail_panel(ui: &mut egui::Ui, app: &mut App) {
    let w = ui.available_width();
    let total_h = ui.available_height();
    let sep = ui.spacing().item_spacing.y + 1.0;
    let bottom_h = (total_h - DETAIL_HEIGHT - sep).max(0.0);

    ui.allocate_ui(egui::vec2(w, DETAIL_HEIGHT), |ui| {
        ui.set_height(DETAIL_HEIGHT);
        egui::ScrollArea::vertical()
            .id_salt("skill_detail")
            .max_height(DETAIL_HEIGHT)
            .show(ui, |ui| {
                if let Some(hover_skill) = app.hover_skill() {
                    detail_panel(ui, hover_skill);
                } else if let Some(click_skill) = app.click_skill() {
                    detail_panel(ui, click_skill);
                }
            });
    });

    ui.separator();

    ui.allocate_ui(egui::vec2(w, bottom_h), |ui| {
        ui.set_height(bottom_h);
        egui::ScrollArea::vertical()
            .id_salt("skill_effects")
            .max_height(bottom_h)
            .show(ui, |ui| {
                ui.label("Skill Effects Panel");
            });
    });
}

/// スキルの詳細を表示する。
fn detail_panel(ui: &mut egui::Ui, skill: &Skill) {
    ui.heading(skill.name());

    ui.separator();

    detail_rich_label(ui, skill);

    ui.separator();

    effects_rich_label(ui, skill);
}

/// スキルの詳細をリッチテキストで表示する。
fn detail_rich_label(ui: &mut egui::Ui, skill: &Skill) {
    let mut job = LayoutJob::default();
    let style = ui.style();
    let font = FontSelection::Default;
    let default_color = style.visuals.text_color();

    let description_parts = skill.description_parts();

    RichText::new(description_parts[0].clone())
        .color(default_color)
        .append_to(&mut job, style.as_ref(), font.clone(), Align::Center);

    RichText::new(description_parts[1].clone())
        .color(COLOR_YELLOW)
        .append_to(&mut job, style.as_ref(), font.clone(), Align::Center);

    RichText::new(description_parts[2].clone())
        .color(default_color)
        .append_to(&mut job, style.as_ref(), font.clone(), Align::Center);

    ui.label(job);
}

/// スキルの効果をリッチテキストで表示する。
fn effects_rich_label(ui: &mut egui::Ui, skill: &Skill) {
    let mut job = LayoutJob::default();
    let style = ui.style();
    let font = FontSelection::Default;
    let default_color = style.visuals.text_color();

    let effects_parts = skill.effects_parts();

    for (i, part) in effects_parts.iter().enumerate() {
        if *skill.level() as u32 == (i + 1) as u32 {
            RichText::new(part)
                .color(COLOR_YELLOW)
                .append_to(&mut job, style.as_ref(), font.clone(), Align::Center);

        } else {
            RichText::new(part)
                .color(default_color)
                .append_to(&mut job, style.as_ref(), font.clone(), Align::Center);
        }

        if i < effects_parts.len() - 1 {
            RichText::new(" / ")
                .color(default_color)
                .append_to(&mut job, style.as_ref(), font.clone(), Align::Center);
        }
    }

    ui.label(job);
}
