use crate::ui::{
    EditorState,
    components::{
        dialog::dialog,
        text_button::{text_button, text_button_highlighted},
    },
    editor::{DialogState, actions::EditorAction},
};
use eframe::egui;

impl DialogState {
    pub(super) fn close_code_buffer_dialog(&mut self, ui: &egui::Ui, state: &mut EditorState) {
        let DialogState::CloseCodeBuffer { panel_id } = self else {
            return;
        };

        let mut should_close = false;
        let modal = dialog(ui, "Unsaved Changes", |ui| {
            ui.label(
                    "You have unsaved changes in the code buffer. Would you like to save changes before closing?",
                );

            ui.horizontal(|ui| {
                if text_button(ui, "cancel", "Cancel").clicked() {
                    should_close = true;
                }

                if text_button(ui, "discard_changes", "Don't Save").clicked() {
                    state
                        .actions
                        .push_action(EditorAction::CloseCodeBuffer(*panel_id));
                    should_close = true;
                }

                if text_button_highlighted(ui, "save_changes", "Save").clicked() {
                    state
                        .actions
                        .push_action(EditorAction::SaveCodeBuffer(*panel_id));
                    state
                        .actions
                        .push_action(EditorAction::CloseCodeBuffer(*panel_id));
                    should_close = true;
                }
            });
        });

        if should_close || modal.should_close() {
            *self = DialogState::None;
        }
    }
}
