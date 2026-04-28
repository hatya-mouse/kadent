use crate::theme;
use eframe::egui;

pub(crate) fn text_input(ui: &mut egui::Ui, value: &mut String) -> egui::Response {
    let dark = ui.visuals().dark_mode;
    let stroke = egui::Stroke::new(1.0, theme::border(dark));

    ui.visuals_mut().extreme_bg_color = theme::tertiary_bg(dark);
    ui.visuals_mut().widgets.inactive.bg_stroke = stroke;
    ui.visuals_mut().widgets.hovered.bg_stroke = stroke;
    ui.visuals_mut().widgets.active.bg_stroke = stroke;
    ui.add(
        egui::TextEdit::singleline(value)
            .margin(egui::vec2(6.0, 4.0))
            .desired_width(ui.available_width())
            .font(egui::FontId::new(
                theme::normal_font_size(),
                egui::FontFamily::Proportional,
            ))
            .text_color(theme::primary_fg(dark)),
    )
}

pub(crate) fn text_input_with_callback<F>(
    ui: &mut egui::Ui,
    mut value: String,
    on_change: F,
) -> egui::Response
where
    F: FnOnce(String),
{
    let dark = ui.visuals().dark_mode;
    let stroke = egui::Stroke::new(1.0, theme::border(dark));

    ui.visuals_mut().extreme_bg_color = theme::tertiary_bg(dark);
    ui.visuals_mut().widgets.inactive.bg_stroke = stroke;
    ui.visuals_mut().widgets.hovered.bg_stroke = stroke;
    ui.visuals_mut().widgets.active.bg_stroke = stroke;

    let response = ui.add(
        egui::TextEdit::singleline(&mut value)
            .margin(egui::vec2(6.0, 4.0))
            .desired_width(ui.available_width())
            .font(egui::FontId::new(
                theme::normal_font_size(),
                egui::FontFamily::Proportional,
            ))
            .text_color(theme::primary_fg(dark)),
    );

    if response.changed() {
        on_change(value);
    }

    response
}
