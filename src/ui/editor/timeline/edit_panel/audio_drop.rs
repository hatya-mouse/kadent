use crate::{
    actions::EditorAction,
    ui::{EditorState, editor::state::EditorUiState, theme},
};
use eframe::egui;
use std::path::PathBuf;

impl EditorState {
    pub(crate) fn audio_dropped(&mut self, path: PathBuf) {
        self.ui_state.timeline_state.last_audio_drop = Some(path);
    }

    pub(crate) fn audio_hovered(&mut self, path: PathBuf) {
        self.ui_state.timeline_state.dragging_audio_file = Some(path);
    }

    pub(super) fn try_resolve_audio_drop(&mut self) {
        let Some(file_path) = self.ui_state.timeline_state.last_audio_drop.take() else {
            return;
        };

        // Check if the file format is supported
        let Some(extension) = file_path.extension() else {
            self.ui_state
                .status_bar_state
                .show_temp_status("File format not supported", theme::error_fg());
            return;
        };
        if extension != "wav" {
            self.ui_state
                .status_bar_state
                .show_temp_status("File format not supported", theme::error_fg());
            return;
        }

        // We have already checked that last_audio_drop is Some, so we can safely unwrap it here
        self.push_action(EditorAction::ImportAudioFile(
            file_path,
            self.ui_state.playhead_tick,
        ));
    }
}

pub(super) fn show_dragged_hint(ui: &mut egui::Ui, ui_state: &mut EditorUiState) {
    // Show the hover overlay for the dragged audio file
    if ui_state.timeline_state.dragging_audio_file.is_none() {
        return;
    }
    ui_state.timeline_state.dragging_audio_file = None;

    // Draw the hint overlay
    ui.painter()
        .rect_filled(ui.content_rect(), 0.0, theme::panel_collapse_overlay());
}
