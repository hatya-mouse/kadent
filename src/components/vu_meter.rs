use eframe::egui;

pub(crate) fn vu_meter(ui: &egui::Ui, value: f32, width: f32, height: f32, corner_radius: f32) {
    let painter = ui.painter();

    let bar_rect = egui::Rect::from_min_size(
        ui.cursor().min,
        egui::vec2(width * value.clamp(0.0, 1.0), height),
    );
    let fill_color = if value < 0.6 {
        egui::Color32::GREEN
    } else if value < 0.85 {
        egui::Color32::YELLOW
    } else {
        egui::Color32::RED
    };
    painter.rect_filled(bar_rect, corner_radius, fill_color);
}
