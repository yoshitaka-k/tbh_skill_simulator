use tbh_skill_simulator::{App, Rendar};

#[warn(clippy::all, rust_2018_idioms)]
// リリースビルド時に Windows でコンソールウィンドウを隠す
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

const WINDOW_WIDTH: f32 = 572.0;
const WINDOW_HEIGHT: f32 = 480.0;

fn main() -> eframe::Result {
    env_logger::init();

    let app = App::new();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_maximize_button(false)
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "TBH Skill Simulator",
        native_options,
        Box::new(|cc| Ok(Box::new(Rendar::new(cc, app)))),
    )
}
