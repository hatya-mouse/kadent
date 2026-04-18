use crate::colors;
use eframe::egui;

pub(super) fn toolbar_group(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(colors::button_bg(ui.visuals().dark_mode))
        .corner_radius(6)
        .inner_margin(1)
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(5.0, 0.0);
            add_contents(ui);
        });
}
