/// egui の [`Response`](egui::Response) にホバー開始検知を追加する。
pub(crate) trait ResponseExt {
    /// ホバー開始フレーム（前フレームはホバーしていなかった）なら `true`。
    fn just_hovered(&self, ui: &egui::Ui) -> bool;
}

impl ResponseExt for egui::Response {
    fn just_hovered(&self, ui: &egui::Ui) -> bool {
        let was_hovered_id = self.id.with("was_hovered");
        let was_hovered = ui
            .ctx()
            .data(|d| d.get_temp::<bool>(was_hovered_id).unwrap_or(false));
        let just_hovered = self.hovered() && !was_hovered;
        ui.ctx()
            .data_mut(|d| d.insert_temp(was_hovered_id, self.hovered()));
        just_hovered
    }
}
