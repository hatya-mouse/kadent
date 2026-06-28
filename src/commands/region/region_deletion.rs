use crate::ui::workspaces::EditorUi;
use kadent_engine::{mixer::TrackID, track::RegionID};

impl EditorUi {
    pub(in crate::commands) fn remove_region(&mut self, track_id: &TrackID, region_id: &RegionID) {
        if let Some(track) = self.proj_ctx.project.get_track_mut(track_id) {
            track.remove_region(region_id);
        }
        if let Some(track_meta) = self.proj_ctx.project_meta.get_track_mut(track_id) {
            track_meta.remove_region(region_id);
        }

        self.ui_state.select_track(*track_id);
        self.modified_project();
    }
}
