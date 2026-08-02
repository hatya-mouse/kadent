use crate::{
    core::metadata::TrackType,
    ui::{theme, workspaces::EditorUi},
};
use kadent_engine::{
    data_types::Ticks,
    mixer::TrackID,
    track::{RegionID, audio_track::AudioTrack, note_track::NoteTrack},
};

impl EditorUi {
    pub(in crate::actions) fn move_region(
        &mut self,
        original_track_id: &TrackID,
        region_id: &RegionID,
        new_track_id: &TrackID,
        new_start: Ticks,
    ) {
        // Move the region to the new start beats
        let Some(original_track) = self.proj_ctx.project.get_track_mut(original_track_id) else {
            return;
        };

        if original_track_id != new_track_id {
            // Move the region to a new track
            let Some(original_track_meta) = self.proj_ctx.project_meta.get_track(original_track_id)
            else {
                return;
            };
            let Some(new_track_meta) = self.proj_ctx.project_meta.get_track(new_track_id) else {
                return;
            };

            match (original_track_meta.track_type, new_track_meta.track_type) {
                (TrackType::Audio, TrackType::Audio) => {
                    if let Some(original_audio_track) =
                        original_track.as_any_mut().downcast_mut::<AudioTrack>()
                        && let Some(region) = original_audio_track.take_region(region_id)
                        && let Some(new_track) = self.proj_ctx.project.get_track_mut(new_track_id)
                        && let Some(new_audio_track) =
                            new_track.as_any_mut().downcast_mut::<AudioTrack>()
                    {
                        new_audio_track.add_region(region);
                    }
                }
                (TrackType::Note, TrackType::Note) => {
                    if let Some(original_note_track) =
                        original_track.as_any_mut().downcast_mut::<NoteTrack>()
                        && let Some(region) = original_note_track.take_region(region_id)
                        && let Some(new_track) = self.proj_ctx.project.get_track_mut(new_track_id)
                        && let Some(new_note_track) =
                            new_track.as_any_mut().downcast_mut::<NoteTrack>()
                    {
                        new_note_track.add_region(region);
                    }
                }
                (original_track_type, new_track_type) => {
                    self.show_temp_status(
                        &format!(
                            "Cannot move {} region to {} track",
                            original_track_type, new_track_type
                        ),
                        theme::error_fg(),
                    );
                }
            }
        } else {
            let Some(original_track_meta) =
                self.proj_ctx.project_meta.get_track_mut(original_track_id)
            else {
                return;
            };
            original_track.move_region(region_id, new_start);

            // Move the region in the region meta too
            if let Some(region_meta) = original_track_meta.get_region_mut(region_id) {
                region_meta.move_region(new_start);
            }
        }

        // Set the region start beats in metadata

        self.modified_project();
    }

    pub(in crate::actions) fn set_region_duration(
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
