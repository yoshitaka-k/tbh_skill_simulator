mod knight_panel;

/// Deserialize/Serialize を derive して、終了時にアプリの状態を保存できるようにする。
#[derive(serde::Deserialize, serde::Serialize)]
// 新しいフィールドを追加したとき、古い状態を復元する際にデフォルト値を使う
#[serde(default)]
pub struct App {
    level: u32,
    skill_points: u32,
}

/// App のデフォルト値を設定する。
impl Default for App {
    fn default() -> Self {
        Self {
            level: 1,
            skill_points: 0,
        }
    }
}

/// App new メソッドを定義する。
impl App {
    /// 最初のフレームの前に一度だけ呼ばれる。
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // ここで `cc.egui_ctx.set_visuals` や `cc.egui_ctx.set_fonts` を使って
        // egui の見た目をカスタマイズすることもできる。
        egui_extras::install_image_loaders(&cc.egui_ctx);

        // 前回のアプリ状態を読み込む（あれば）。
        // これを動かすには `persistence` フィーチャーを有効にする必要がある。
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

/// eframe::App save メソッドを定義する。
impl eframe::App for App {
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
            knight_panel::hero::hero_panel(ui);

            ui.separator();

            knight_panel::level::level_panel(ui, self);

            ui.separator();

            knight_panel::skill::skill_list_panel(ui, self);

            // デバッグビルドの警告を表示する。
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
