use crate::ui::theme;
use eframe::egui;

/// A card like button that shows a background color when hovered.
pub(crate) fn card_button_enabled<R>(
    enabled: bool,
    ui: &mut egui::Ui,
    id: egui::Id,
    desired_size: Option<egui::Vec2>,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::Response {
    let frame_response = ui
        .scope(|ui| {
            if !enabled {
                ui.disable();
            }

            if let Some(size) = desired_size {
                ui.set_min_size(size);
                ui.set_max_size(size);
            } else {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
            }

            egui::Frame::new()
                .corner_radius(6.0)
                .stroke(theme::border(ui.visuals().dark_mode))
                .inner_margin(4.0)
                .multiply_with_opacity(if enabled { 1.0 } else { 0.5 })
                .show(ui, add_contents)
                .response
        })
        .inner;

    if enabled {
        // Get hover response from the inner contents
        let response = ui.interact(frame_response.rect, id, egui::Sense::click());

        // Show background color when pressed or hovered
        if response.is_pointer_button_down_on() {
            ui.painter().rect_filled(
                response.rect,
                egui::CornerRadius::same(6),
                theme::card_button_pressed(ui.visuals().dark_mode),
            );
        } else if response.hovered() {
            ui.painter().rect_filled(
                response.rect,
                egui::CornerRadius::same(6),
                theme::card_button_hovered(ui.visuals().dark_mode),
            );
        }

        response
    } else {
        ui.interact(frame_response.rect, id, egui::Sense::empty())
    }
}

/// A card like button that shows a background color when hovered.
pub(crate) fn card_button<R>(
    ui: &mut egui::Ui,
    id: egui::Id,
    desired_size: Option<egui::Vec2>,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::Response {
    card_button_enabled(true, ui, id, desired_size, add_contents)
}
