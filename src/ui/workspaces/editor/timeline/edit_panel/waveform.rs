use crate::ui::workspaces::EditorUi;
use eframe::egui;
use kadent_engine::{mixer::TrackID, track::RegionID};

impl EditorUi {
    pub(super) fn draw_waveform_in(
        &mut self,
        track_id: TrackID,
        region_id: RegionID,
        rect: &egui::Rect,
    ) {
        let Some(waveform_lod) = self
            .ui_state
            .timeline_state
            .waveforms
            .get(&(track_id, region_id))
        else {
            return;
        };
    }
}
