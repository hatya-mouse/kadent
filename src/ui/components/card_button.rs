use crate::ui::theme;
use eframe::egui;

pub(crate) fn card_button<R>(
    ui: &mut egui::Ui,
    id: egui::Id,
    desired_size: Option<egui::Vec2>,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::Response {
    let frame_response = ui
        .allocate_ui_with_layout(
            desired_size.unwrap_or_else(|| egui::vec2(ui.available_width(), 0.0)),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                egui::Frame::new()
                    .corner_radius(8.0)
                    .stroke(theme::border(ui.visuals().dark_mode))
                    .inner_margin(8.0)
                    .show(ui, add_contents)
                    .response
            },
        )
        .inner;

    // Get hover response from the inner contents
    let response = ui.interact(frame_response.rect, id, egui::Sense::click());

    // Show background color when hovered
    if response.hovered() {
        ui.painter().rect_filled(
            response.rect,
            egui::CornerRadius::same(6),
            if ui.visuals().dark_mode {
                egui::Color32::from_white_alpha(10)
            } else {
                egui::Color32::from_black_alpha(10)
            },
        );
    }

    response
}
