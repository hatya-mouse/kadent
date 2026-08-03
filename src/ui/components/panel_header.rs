use crate::{consts::PANEL_HEADER_HEIGHT, ui::theme};
use eframe::egui;

pub(crate) fn panel_header<R>(
    ui: &mut egui::Ui,
    inner_margin: egui::Margin,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    let response = egui::Panel::top(ui.id().with("header"))
        .frame(
            egui::Frame::new()
                .fill(theme::tertiary_bg(ui.visuals().dark_mode))
                .inner_margin(inner_margin),
        )
        .exact_size(PANEL_HEADER_HEIGHT)
        .show_separator_line(false)
        .show_inside(ui, |ui| {
            ui.spacing_mut().button_padding = egui::vec2(0.0, 0.0);
            ui.horizontal_centered(|ui| add_contents(ui)).inner
        });

    let stroke = theme::border(ui.visuals().dark_mode);
    let rect = response.response.rect;
    ui.painter().hline(rect.x_range(), rect.bottom(), stroke);

    response
}
