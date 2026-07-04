use egui::{Align, FontSelection, RichText, text::LayoutJob};

use super::COLOR_YELLOW;
use crate::hero::{Hero, Skill};

/// スキルの詳細を表示する。
pub(crate) fn detail_panel(ui: &mut egui::Ui, skill: &Skill) {
    ui.heading(skill.name());

    ui.separator();

    detail_rich_label(ui, skill);

    ui.separator();

    effects_rich_label(ui, skill);
}

/// スキルの効果を表示する。
pub(crate) fn effects_panel(ui: &mut egui::Ui, hero: &Hero) {
    ui.heading("Skill Effects");

    ui.separator();

    for (_, skills) in hero.skill_list().iter() {
        for (i, skill) in skills.iter().enumerate() {
            if *skill.level() > 0 {
                detail_rich_label(ui, skill);

                ui.separator();
            }
        }
    }

    ui.add_space(10.0);
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
