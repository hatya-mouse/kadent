use crate::ui::theme;
use eframe::egui;

pub(crate) fn toolbar_icon_button(ui: &mut egui::Ui, image: egui::Image) -> egui::Response {
    ui.add_sized(
        [40.0, 28.0],
        egui::Button::image(
            image
                .fit_to_exact_size(egui::vec2(24.0, 24.0))
                .tint(theme::primary_fg(ui.visuals().dark_mode)),
        )
        .corner_radius(6),
    )
}

pub(crate) fn small_icon_button(ui: &mut egui::Ui, image: egui::Image) -> egui::Response {
    ui.add_sized(
        [28.0, 24.0],
        egui::Button::image(
            image
                .fit_to_exact_size(egui::vec2(20.0, 20.0))
                .tint(theme::primary_fg(ui.visuals().dark_mode)),
        )
        .corner_radius(6),
    )
}
