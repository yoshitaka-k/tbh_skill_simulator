pub mod hero;
pub mod hero_panel;
pub mod level;
pub mod skill;
pub mod skill_panel;

use egui::Color32;

const COLOR_YELLOW: Color32 = Color32::from_rgb(200, 170, 80);
const COLOR_GRAY: Color32 = Color32::from_gray(50);
const HOVER_COLOR_YELLOW: Color32 = Color32::from_rgb(255, 220, 100);
const HOVER_COLOR_GRAY: Color32 = Color32::from_gray(120);
const HOVER_COLOR_GRAY_NOT_ACTIVE: Color32 = Color32::from_gray(80);
const HERO_IMAGE_GRAY_SCALE: Color32 = Color32::from_gray(80);
