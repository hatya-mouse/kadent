use crate::ui::EditorState;
use kadent_engine::{mixer::TrackID, track::RegionID};

impl EditorState {
    pub(in crate::actions) fn remove_region(&mut self, track_id: &TrackID, region_id: &RegionID) {
        if let Some(track) = self.project.data.get_track_mut(track_id) {
            track.remove_region(region_id);
        }
        if let Some(track_meta) = self.project.meta.get_track_mut(track_id) {
            track_meta.remove_region(region_id);
        }

        self.selection.select_track(*track_id);
        self.modified_project();
    }
}
