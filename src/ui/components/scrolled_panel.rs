use eframe::egui::{self, UiBuilder};

pub(crate) fn scrolled_panel<R>(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    scroll_y: f32,
    content: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    // Clamp to the parent's clip rect so content never bleeds outside the enclosing panel,
    // even when rect (e.g. track_list_width) exceeds the available panel width.
    let rect = rect.intersect(ui.clip_rect());

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
