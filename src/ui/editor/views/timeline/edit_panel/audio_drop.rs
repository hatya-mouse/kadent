use crate::{
    ui::editor::actions::EditorAction,
    ui::{EditorState, theme},
};
use eframe::egui;
use std::path::PathBuf;

impl EditorState {
    pub(crate) fn audio_dropped(&mut self, path: PathBuf) {
        self.views.timeline.last_audio_drop = Some(path);
    }

    pub(crate) fn audio_hovered(&mut self, path: PathBuf) {
        self.views.timeline.dragging_audio_file = Some(path);
    }

    pub(super) fn try_resolve_audio_drop(&mut self) {
        let Some(file_path) = self.views.timeline.last_audio_drop.take() else {
            return;
        };

        // Check if the file format is supported
        let Some(extension) = file_path.extension() else {
            self.views
                .status_bar
                .show_temp_status("File format not supported", theme::error_fg());
            return;
        };
        if extension != "wav" {
            self.views
                .status_bar
                .show_temp_status("File format not supported", theme::error_fg());
            return;
        }

        // We have already checked that last_audio_drop is Some, so we can safely unwrap it here
        self.actions.push_action(EditorAction::ImportAudioFile(
            file_path,
            self.transport.playhead_tick,
        ));
    }

    pub(super) fn show_dragged_hint(&mut self, ui: &mut egui::Ui) {
        // Show the hover overlay for the dragged audio file
        if self.views.timeline.dragging_audio_file.is_none() {
            return;
        }
        self.views.timeline.dragging_audio_file = None;

        // Draw the hint overlay
        ui.painter()
            .rect_filled(ui.content_rect(), 0.0, theme::panel_collapse_overlay());
    }
}
