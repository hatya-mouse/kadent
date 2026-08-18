mod item;

use crate::ui::{EditorState, theme};
use eframe::egui;
use item::draw_error_item;

impl EditorState {
    pub(in crate::ui::editor) fn error_list(&self, ui: &mut egui::Ui) {
        ui.painter().rect_filled(
            ui.available_rect_before_wrap(),
            0.0,
            theme::secondary_bg(ui.visuals().dark_mode),
        );

        egui::ScrollArea::vertical()
            .auto_shrink(false)
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                for error in &self.views.errors {
                    draw_error_item(ui, error);
                }
            });
    }
}
