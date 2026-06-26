use getset::{Getters, Setters};

mod panel;

use crate::app;
use panel::{hero, level, skill};

/// UI の状態を保存する。
#[derive(Getters, Setters, serde::Deserialize, serde::Serialize)]
pub struct Rendar {
    #[getset(get = "pub", set = "pub")]
    app: app::App,
}

/// UI new メソッドを定義する。
impl Rendar {
    /// 最初のフレームの前に一度だけ呼ばれる。
    pub fn new(cc: &eframe::CreationContext<'_>, app: app::App) -> Self {
        // ここで `cc.egui_ctx.set_visuals` や `cc.egui_ctx.set_fonts` を使って
        // egui の見た目をカスタマイズすることもできる。
        egui_extras::install_image_loaders(&cc.egui_ctx);

        // 前回のアプリ状態を読み込む（あれば）。
        // これを動かすには `persistence` フィーチャーを有効にする必要がある。
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or(Rendar { app })
        } else {
            Rendar { app }
        }
    }
}

/// eframe::App save メソッドを定義する。
impl eframe::App for Rendar {
    /// 終了前にフレームワークから呼ばれ、状態を保存する。
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// UI の再描画が必要なたびに呼ばれる（秒間に何度も呼ばれることがある）。
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // グローバルなスタイルをカスタマイズする。
        ui.ctx().global_style_mut(|style| {
            // ラベルを選択できないようにする。
            style.interaction.selectable_labels = false;
        });

        // 中央パネルを表示する。
        egui::CentralPanel::default().show_inside(ui, |ui| {
            hero::hero_list_panel(ui, &mut self.app);

            ui.separator();

            level::level_panel(ui, &mut self.app);

            ui.separator();

            skill::skill_list_panel(ui, &mut self.app);

            // デバッグビルドの警告を表示する。
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
