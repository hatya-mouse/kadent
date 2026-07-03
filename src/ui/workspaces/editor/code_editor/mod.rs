mod file_browser;
mod kasl_editor;

use crate::ui::workspaces::EditorUi;
use eframe::egui;

impl EditorUi {
    pub(super) fn code_editor(&mut self, ui: &mut egui::Ui) {
        // Load the code from the temporary storage
        let buffer_index_id = ui.id().with("buffer_index");
        let buffer_index: Option<usize> = ui.data_mut(|data| data.get_temp(buffer_index_id));

        if let Some(buffer_index) = buffer_index {
            egui::Panel::left(ui.id().with("code_editor_left")).show_inside(ui, |ui| {
                self.file_browser(ui, buffer_index);
            });

            egui::CentralPanel::default()
                .frame(egui::Frame::new())
                .show_inside(ui, |ui| {
                    self.kasl_editor(ui, buffer_index);
                });
        } else {
            let new_buffer_index = self.ui_state.code_editor_state.code_buffers.len();
            self.ui_state.code_editor_state.code_buffers.push(None);
            ui.data_mut(|data| data.insert_temp(buffer_index_id, new_buffer_index));
        }
    }
}
