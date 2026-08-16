mod note_grid;

use crate::{
    core::metadata::TrackType,
    ui::{EditorState, editor::piano_roll::note_grid::note_grid, theme},
};
use eframe::egui;

pub fn piano_roll(ui: &mut egui::Ui, state: &mut EditorState) {
    let Some((track_id, region_id)) = state.ui_state.selection.track_and_region_id() else {
        ui.label("Select a note region to edit");
        return;
    };

    // Get the region
    if state
        .ui_state
        .proj_ctx
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
        .fill(theme::secondary_bg(ui.visuals().dark_mode))
        .show(ui, |ui| {
            note_grid(ui, state, grid_rect, track_id, region_id);
        });
}
