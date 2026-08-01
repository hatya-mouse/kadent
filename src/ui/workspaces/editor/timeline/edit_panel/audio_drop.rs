use crate::{actions::EditorAction, ui::workspaces::EditorUi};
use eframe::egui;
use kadent_engine::mixer::TrackID;
use std::path::PathBuf;

impl EditorUi {
    pub(crate) fn audio_dropped(&mut self, path: PathBuf, drop_pos: egui::Pos2) {
        self.ui_state.last_audio_drop = Some((path, drop_pos));
    }

    pub(super) fn try_resolve_audio_drop(&mut self, track_id: &TrackID, row_rect: egui::Rect) {
        let Some((_, pos)) = self.ui_state.last_audio_drop else {
            return;
        };
        if !row_rect.contains(pos) {
            return;
        }

        // We have already checked that last_audio_drop is Some, so we can safely unwrap it here
        let (path, pos) = self.ui_state.last_audio_drop.take().unwrap();
        let start = self.x_to_ticks(pos.x, row_rect);
        self.push_action(EditorAction::ImportAudioFile(*track_id, start, path));
    }
}
