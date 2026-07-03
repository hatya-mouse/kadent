mod file_browser;
mod kasl_editor;

use crate::ui::workspaces::EditorUi;
use eframe::egui;

impl EditorUi {
    pub(super) fn code_editor(&mut self, ui: &mut egui::Ui, panel_id: egui::Id) {
        // Ensure a buffer entry exists for this panel
        self.ui_state
            .code_editor_state
            .code_buffers
            .entry(panel_id)
            .or_insert(None);

        egui::Panel::left(panel_id.with("code_editor_left")).show_inside(ui, |ui| {
            self.file_browser(ui, panel_id);
        });

        egui::CentralPanel::default()
            .frame(egui::Frame::new())
            .show_inside(ui, |ui| {
                self.kasl_editor(ui, panel_id);
            });
    }
}
