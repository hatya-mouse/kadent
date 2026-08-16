use crate::ui::EditorState;
use kadent_engine::{mixer::TrackID, track::RegionID};

impl EditorState {
    pub(in crate::actions) fn remove_region(&mut self, track_id: &TrackID, region_id: &RegionID) {
        if let Some(track) = self.ui_state.proj_ctx.project.get_track_mut(track_id) {
            track.remove_region(region_id);
        }
        if let Some(track_meta) = self.ui_state.proj_ctx.project_meta.get_track_mut(track_id) {
            track_meta.remove_region(region_id);
        }

        self.ui_state.select_track(*track_id);
        self.modified_project();
    }
}
