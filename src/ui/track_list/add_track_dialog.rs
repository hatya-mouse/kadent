use crate::app::KnodiqApp;
use eframe::egui;

impl KnodiqApp {
    pub(crate) fn add_track_dialog(&mut self, ctx: &egui::Context) {
        let modal = egui::Modal::new(egui::Id::new("add_track")).show(ctx, |ui| {
            ui.label("Select track type");
            if ui.button("Note Track").clicked() {
                self.ui_state.show_add_track_dialog = false;
            }
            if ui.button("Cancel").clicked() {
                self.ui_state.show_add_track_dialog = false;
            }
        });

        if modal.should_close() {
            self.ui_state.show_add_track_dialog = false;
        }
    }
}
