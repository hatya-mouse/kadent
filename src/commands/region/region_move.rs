use crate::ui::workspaces::EditorUi;
use kadent_engine::{data_types::Ticks, mixer::TrackID, track::RegionID};

impl EditorUi {
    pub(in crate::commands) fn move_region(
        &mut self,
        track_id: &TrackID,
        region_id: &RegionID,
        new_start: Ticks,
    ) {
        // Move the region to the new start beats
        if let Some(track) = self.proj_ctx.project.get_track_mut(track_id) {
            track.move_region(region_id, new_start);
        }

        // Set the region start beats in metadata
        if let Some(track_meta) = self.proj_ctx.project_meta.get_track_mut(track_id)
            && let Some(region_meta) = track_meta.get_region_mut(region_id)
        {
            region_meta.move_region(new_start);
        }

        self.modified_project();
    }

    pub(in crate::commands) fn set_region_duration(
        &mut self,
        track_id: &TrackID,
        region_id: &RegionID,
        new_duration: Ticks,
    ) {
        // Move the region to the new start beats
        if let Some(track) = self.proj_ctx.project.get_track_mut(track_id) {
            track.set_region_duration(region_id, new_duration);
        }

        // Set the region start beats in metadata
        if let Some(track_meta) = self.proj_ctx.project_meta.get_track_mut(track_id)
            && let Some(region_meta) = track_meta.get_region_mut(region_id)
        {
            region_meta.set_duration(new_duration);
        }

        self.modified_project();
    }
}
