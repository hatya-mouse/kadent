use crate::ui::{
    EditorState,
    components::{
        dialog::{dialog, dialog_bold_label},
        text_button::{text_button, text_button_enabled},
        text_input::text_input,
    },
    editor::{DialogState, actions::EditorAction},
};
use eframe::egui;

impl DialogState {
    pub(super) fn rename_file_dialog(&mut self, ui: &egui::Ui, state: &mut EditorState) {
        let DialogState::RenameFile { path, new_name } = self else {
            return;
        };

        let mut should_close = false;
        let modal = dialog(ui, "Rename Item", 6, |ui| {
            dialog_bold_label(ui, format!("Rename {} to:", path.display()));
            text_input(ui, new_name);

            ui.add_space(4.0);

            ui.horizontal(|ui| {
                if text_button(ui, "cancel", "Cancel").clicked() {
                    should_close = true;
                }

                let is_name_valid = validate_file_name(new_name);
                if text_button_enabled(is_name_valid, true, ui, "rename", "Rename").clicked() {
                    let Some(parent) = path.parent() else {
                        return;
                    };
                    let new_path = parent.join(new_name);
                    state
                        .actions
                        .push_action(EditorAction::MoveFile(path.clone(), new_path));
                    state.actions.push_action(EditorAction::UpdateDirCache);
                    should_close = true;
                }
            });
        });

        if should_close || modal.should_close() {
            *self = DialogState::None;
        }
    }
}

fn validate_file_name(name: &str) -> bool {
    !name.trim().is_empty() && !name.contains(std::path::MAIN_SEPARATOR) && !name.starts_with('.')
}
