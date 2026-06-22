use crate::ui::theme;
use eframe::egui;

const CORNER_RADIUS: f32 = 6.0;

pub(crate) fn toolbar_icon_button(ui: &mut egui::Ui, image: egui::Image) -> egui::Response {
    toolbar_icon_button_colored(ui, image, None)
}

pub(crate) fn toolbar_icon_button_colored(
    ui: &mut egui::Ui,
    image: egui::Image,
    bg_color: Option<egui::Color32>,
) -> egui::Response {
    let response = ui.add_sized(
        [40.0, 28.0],
        egui::Button::image(
            image
                .fit_to_exact_size(egui::vec2(24.0, 24.0))
                .tint(theme::primary_fg(ui.visuals().dark_mode)),
        )
        .fill(bg_color.unwrap_or(egui::Color32::TRANSPARENT))
        .corner_radius(CORNER_RADIUS),
    );

    if response.is_pointer_button_down_on() {
        ui.painter()
            .rect_filled(response.rect, CORNER_RADIUS, theme::icon_button_active());
    } else if response.hovered() {
        ui.painter()
            .rect_filled(response.rect, CORNER_RADIUS, theme::icon_button_hovered());
    }

    response
}

pub(crate) fn small_icon_button(ui: &mut egui::Ui, image: egui::Image) -> egui::Response {
    let response = ui.add_sized(
        [28.0, 24.0],
        egui::Button::image(
            image
                .fit_to_exact_size(egui::vec2(20.0, 20.0))
                .tint(theme::primary_fg(ui.visuals().dark_mode)),
        )
        .corner_radius(CORNER_RADIUS),
    );

    if response.is_pointer_button_down_on() {
        ui.painter()
            .rect_filled(response.rect, CORNER_RADIUS, theme::icon_button_active());
    } else if response.hovered() {
        ui.painter()
            .rect_filled(response.rect, CORNER_RADIUS, theme::icon_button_hovered());
    }

    response
}
