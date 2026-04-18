mod note_grid;
mod utils;

use crate::{app::KnodiqApp, colors, ui_state::dialog_state::TrackType};
use eframe::egui;

impl KnodiqApp {
    pub(in crate::ui) fn piano_roll(&mut self, ui: &mut egui::Ui) {
        let Some((track_id, region_id)) = self.ui_state.editor_state.selected_region else {
            ui.label("Select a note region to edit");
            return;
        };

        // Get the region
        if self
            .project_meta
            .get_track(&track_id)
            .is_none_or(|track| track.track_type != TrackType::Note)
        {
            ui.label("Select a note region to edit");
            return;
        }

        let total_rect = ui.available_rect_before_wrap();

        // Draw notes
        let grid_rect = egui::Rect::from_min_max(total_rect.min, total_rect.max);
        egui::Frame::new()
            .fill(colors::secondary_bg(ui.visuals().dark_mode))
            .show(ui, |ui| {
                self.note_grid(ui, grid_rect, track_id, region_id);
            });
    }
}
