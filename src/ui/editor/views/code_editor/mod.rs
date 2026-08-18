mod file_browser;
mod kasl_editor;
mod state;

pub(crate) use state::CodeEditorState;

use crate::ui::EditorState;
use eframe::egui;

impl EditorState {
    pub(in crate::ui::editor) fn code_editor(&mut self, ui: &mut egui::Ui, panel_id: egui::Id) {
        // Ensure a buffer entry exists for this panel
        self.views
            .code_editor
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
