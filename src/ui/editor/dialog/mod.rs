mod change_code_buffer;
mod rename_file;
mod track_dialog;

use crate::ui::{EditorState, editor::DialogState};
use eframe::egui;

impl DialogState {
    pub(super) fn dialog(&mut self, ui: &egui::Ui, state: &mut EditorState) {
        match self {
            DialogState::None => (),
            DialogState::AddTrack { .. } => {
                self.track_dialog(ui, state);
            }
            DialogState::ChangeCodeBuffer { .. } => {
                self.change_code_buffer_dialog(ui, state);
            }
            DialogState::RenameFile { .. } => {
                self.rename_file_dialog(ui, state);
            }
        }
    }
}
