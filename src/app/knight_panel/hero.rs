/// キャラクターパネルを表示する。
pub(crate) fn hero_panel(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.add(egui::Image::new(egui::include_image!("../../../assets/images/hero_knight.png"))
                .fit_to_original_size(1.0)
            );
            ui.heading("Knight");
        });
    });
}
