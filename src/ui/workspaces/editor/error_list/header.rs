use crate::ui::{theme, workspaces::EditorUi};
use eframe::egui;

impl EditorUi {
    pub(super) fn draw_error_list_header(&mut self, ui: &mut egui::Ui) {
        let response = egui::Frame::new()
            .fill(theme::tertiary_bg(ui.visuals().dark_mode))
            .inner_margin(egui::Margin::symmetric(8, 4))
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.horizontal(|ui| {
                    let _ = ui;
                });
            });

        let rect = response.response.rect;
        ui.painter().line_segment(
            [
                rect.left_bottom() - egui::vec2(0.0, 0.5),
                rect.right_bottom() - egui::vec2(0.0, 0.5),
            ],
            egui::Stroke::new(0.5, theme::border_color(ui.visuals().dark_mode)),
        );
    }
}
