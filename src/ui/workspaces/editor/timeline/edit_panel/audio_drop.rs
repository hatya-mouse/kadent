use crate::{
    actions::EditorAction,
    ui::{theme, workspaces::EditorUi},
};
use eframe::egui;
use kadent_engine::mixer::TrackID;
use std::path::PathBuf;

const HINT_HALF_WIDTH: f32 = 4.0;

impl EditorUi {
    pub(crate) fn audio_dropped(&mut self, path: PathBuf, drop_pos: egui::Pos2) {
        self.ui_state.timeline_state.last_audio_drop = Some((path, drop_pos));
    }

    pub(crate) fn audio_hovered(&mut self, path: PathBuf, drop_pos: egui::Pos2) {
        self.ui_state.timeline_state.dragging_audio_file = Some((path, drop_pos));
    }

    pub(super) fn try_resolve_audio_drop(&mut self, track_id: &TrackID, row_rect: egui::Rect) {
        let Some((_, pos)) = self.ui_state.timeline_state.last_audio_drop else {
            return;
        };
        if !row_rect.contains(pos) {
            return;
        }

        // We have already checked that last_audio_drop is Some, so we can safely unwrap it here
        let (path, pos) = self.ui_state.timeline_state.last_audio_drop.take().unwrap();
        let start = self.x_to_ticks(pos.x, row_rect);
        self.push_action(EditorAction::ImportAudioFile(*track_id, start, path));
    }

    pub(super) fn show_dragged_hint(&mut self, ui: &mut egui::Ui, row_rect: egui::Rect) {
        // Show the hover overlay for the dragged audio file
        let Some((_, pos)) = self.ui_state.timeline_state.dragging_audio_file else {
            return;
        };
        if !row_rect.contains(pos) {
            return;
        }
        self.ui_state.timeline_state.dragging_audio_file = None;

        // Draw the hint overlay
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                egui::pos2(pos.x - HINT_HALF_WIDTH, row_rect.min.y),
                egui::pos2(pos.x + HINT_HALF_WIDTH, row_rect.max.y),
            ),
            0.0,
            theme::selected_fg(),
        );
    }
}
