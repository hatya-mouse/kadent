mod kasl_editor;

use crate::ui::workspaces::EditorUi;
use eframe::egui;

impl EditorUi {
    pub(super) fn code_editor(&mut self, ui: &mut egui::Ui) {
        // Load the code from the temporary storage
        let buffer_index_id = ui.id().with("buffer_index");
        let buffer_index: Option<usize> = ui.data_mut(|data| data.get_temp(buffer_index_id));

        egui::Panel::left(ui.id().with("code_editor_left")).show_inside(ui, |_| {
            // FILE BROWSER
        });

        egui::Panel::right(ui.id().with("code_editor_right")).show_inside(ui, |ui| {
            if let Some(buffer_index) = buffer_index {
                self.kasl_editor(ui, buffer_index);
            }
        });
    }
}
