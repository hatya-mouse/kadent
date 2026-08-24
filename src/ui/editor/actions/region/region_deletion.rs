use crate::core::audio_engine::{mixer::TrackID, track::RegionID};
use crate::ui::editor::EditorUi;

impl EditorUi {
    pub(crate) fn remove_region(&mut self, track_id: &TrackID, region_id: &RegionID) {
        if let Some(track) = self.state.project.data.get_track_mut(track_id) {
            track.remove_region(region_id);
        }
        if let Some(track_meta) = self.state.project.meta.get_track_mut(track_id) {
            track_meta.remove_region(region_id);
        }

        self.state.selection.select_track(*track_id);
        self.state.actions.modified_project();
    }
}
