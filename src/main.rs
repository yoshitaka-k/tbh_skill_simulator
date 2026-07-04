use eframe::icon_data::from_png_bytes;

use tbh_skill_simulator::{App, Rendar};

#[warn(clippy::all, rust_2018_idioms)]
// リリースビルド時に Windows でコンソールウィンドウを隠す
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

const APP_NAME: &str = "TBH Skill Simulator";

const ICON: &[u8] = include_bytes!("../assets/icon.png");

const WINDOW_WIDTH: f32 = 572.0;
const WINDOW_HEIGHT: f32 = 480.0;

fn main() -> eframe::Result {
    env_logger::init();

    let app = App::new();

    let icon = from_png_bytes(ICON).expect("Invalid icon PNG");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_maximize_button(false)
            .with_resizable(false)
            .with_icon(icon),
        ..Default::default()
    };
    eframe::run_native(
        APP_NAME,
        native_options,
        Box::new(|cc| Ok(Box::new(Rendar::new(cc, app)))),
    )
}
