mod kasl_editor;

use crate::ui::workspaces::EditorUi;
use eframe::egui;

impl EditorUi {
    pub(super) fn code_editor(&mut self, ui: &mut egui::Ui) {
        // Load the code from the temporary storage
        let buffer_code_id = ui.id().with("buffer_code");
        let Some(mut code) = ui.data_mut(|data| data.get_temp(buffer_code_id)) else {
            return;
        };

        self.kasl_editor(ui, &mut code);

        // Save the code back to the temporary storage
        ui.data_mut(|data| data.insert_temp(buffer_code_id, code));
    }
}
