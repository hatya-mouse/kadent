use crate::ui::theme;
use eframe::egui;

const CORNER_RADIUS: f32 = 6.0;
pub(crate) const SMALL_TOOLBAR_ICON_SIZE: egui::Vec2 = egui::vec2(20.0, 20.0);

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

pub(crate) fn small_toolbar_button_highlighted<R>(
    ui: &mut egui::Ui,
    is_highlighted: bool,
    add_contents: impl FnOnce(&mut egui::Ui, egui::Color32) -> R,
) -> egui::InnerResponse<R> {
    let mut frame = egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(4, 2))
        .corner_radius(CORNER_RADIUS)
        .fill(egui::Color32::TRANSPARENT)
        .begin(ui);

    let fg = if is_highlighted {
        theme::selected_fg()
    } else {
        theme::primary_fg(ui.visuals().dark_mode)
    };

    let inner = add_contents(&mut frame.content_ui, fg);
    let frame_response = frame.allocate_space(ui);
    let response = ui.interact(frame_response.rect, frame_response.id, egui::Sense::click());

    let bg = if is_highlighted {
        theme::selected_bg()
    } else if response.is_pointer_button_down_on() {
        theme::icon_button_active()
    } else if response.hovered() {
        theme::icon_button_hovered()
    } else {
        egui::Color32::TRANSPARENT
    };

    frame.frame.fill = bg;
    frame.paint(ui);

    egui::InnerResponse::new(inner, response)
}

pub(crate) fn small_toolbar_button<R>(
    ui: &mut egui::Ui,
    add_contents: impl FnOnce(&mut egui::Ui, egui::Color32) -> R,
) -> egui::InnerResponse<R> {
    small_toolbar_button_highlighted(ui, false, add_contents)
}

pub(crate) fn small_icon_button_highlighted(
    ui: &mut egui::Ui,
    image: egui::Image,
    is_highlighted: bool,
    tinted: bool,
) -> egui::Response {
    small_toolbar_button_highlighted(ui, is_highlighted, |ui, fg| {
        let image = if tinted { image.tint(fg) } else { image };
        ui.add(image.fit_to_exact_size(SMALL_TOOLBAR_ICON_SIZE))
    })
    .response
}

pub(crate) fn small_icon_button(
    ui: &mut egui::Ui,
    image: egui::Image,
    tinted: bool,
) -> egui::Response {
    small_icon_button_highlighted(ui, image, false, tinted)
}
