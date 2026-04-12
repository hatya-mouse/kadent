mod note_grid;
mod utils;

use crate::{app::KnodiqApp, colors, ui_state::dialog_state::TrackType};
use eframe::egui;

impl KnodiqApp {
    pub(super) fn piano_roll(&mut self, ui: &mut egui::Ui) {
        let Some((track_id, region_id)) = self.ui_state.selected_region else {
            ui.label("Select a note region to edit");
            return;
        };

        // Get the region
        if self
            .project_meta
            .get_track(&track_id)
            .is_none_or(|track| track.track_type != TrackType::NoteTrack)
        {
            ui.label("Select a note region to edit");
            return;
        }

        let total_rect = ui.available_rect_before_wrap();

        // Draw notes
        let grid_rect = egui::Rect::from_min_max(
            egui::pos2(total_rect.min.x, total_rect.min.y),
            total_rect.max,
        );
        egui::Frame::new()
            .fill(colors::secondary_bg(ui.visuals().dark_mode))
            .show(ui, |ui| {
                self.note_grid(ui, grid_rect, track_id, region_id);
            });
    }
}
