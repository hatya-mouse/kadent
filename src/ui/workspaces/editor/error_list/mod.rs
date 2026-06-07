mod header;
mod item;

use crate::ui::{theme, workspaces::EditorUi};
use eframe::egui;
use item::draw_error_item;

impl EditorUi {
    pub fn error_list(&mut self, ui: &mut egui::Ui) {
        self.draw_error_list_header(ui);

        ui.painter().rect_filled(
            ui.available_rect_before_wrap(),
            0.0,
            theme::secondary_bg(ui.visuals().dark_mode),
        );

        egui::ScrollArea::vertical()
            .auto_shrink(false)
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                for error in &self.errors {
                    draw_error_item(ui, error);
                }
            });
    }
}
