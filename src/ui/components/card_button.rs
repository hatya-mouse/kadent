use crate::ui::theme;
use eframe::egui;

const CARD_BUTTON_MARGIN: egui::Margin = egui::Margin::symmetric(8, 3);

/// A card like button that shows a background color when hovered.
pub(crate) fn card_button_enabled<R>(
    enabled: bool,
    highlighted: bool,
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

            if highlighted {
                ui.visuals_mut().widgets.active.fg_stroke =
                    egui::Stroke::new(1.0, theme::selected_fg());
                ui.visuals_mut().widgets.inactive.fg_stroke =
                    egui::Stroke::new(1.0, theme::selected_fg());
                ui.visuals_mut().widgets.open.fg_stroke =
                    egui::Stroke::new(1.0, theme::selected_fg());
                ui.visuals_mut().widgets.hovered.fg_stroke =
                    egui::Stroke::new(1.0, theme::selected_fg());
                ui.visuals_mut().widgets.noninteractive.fg_stroke =
                    egui::Stroke::new(1.0, theme::selected_fg());
            }

            egui::Frame::new()
                .corner_radius(6.0)
                .fill(if highlighted {
                    theme::selected_bg()
                } else {
                    egui::Color32::TRANSPARENT
                })
                .stroke(theme::border(ui.visuals().dark_mode))
                .inner_margin(CARD_BUTTON_MARGIN)
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
    card_button_enabled(true, false, ui, id, desired_size, add_contents)
}

/// A card like button that is highlighted.
pub(crate) fn card_button_highlighted<R>(
    ui: &mut egui::Ui,
    id: egui::Id,
    desired_size: Option<egui::Vec2>,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::Response {
    card_button_enabled(true, true, ui, id, desired_size, add_contents)
}
