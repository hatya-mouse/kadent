use eframe::egui::{self, UiBuilder};

pub(crate) fn scrolled_panel<R>(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    scroll_y: f32,
    content: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    let mut result = None;
    ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
        ui.set_clip_rect(rect);
        let offset_rect = rect.translate(egui::vec2(0.0, -scroll_y));
        ui.scope_builder(UiBuilder::new().max_rect(offset_rect), |ui| {
            result = Some(content(ui));
        });
    });
    result.unwrap()
}
