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

            // Check if the destination track is of the same type as the original track
            if original_track_meta.track_type != new_track_meta.track_type {
                self.show_temp_status(
                    &format!(
                        "Cannot move {} region to {} track",
                        original_track_meta.track_type.fmt_lowercase(),
                        new_track_meta.track_type.fmt_lowercase()
                    ),
                    theme::error_fg(),
                );
                return;
            }

            // Store the new region id after moving the region to the new track
            // This variable is also used for checking whether moving the region was successful or not
            let mut new_region_id = None;
            match original_track_meta.track_type {
                TrackType::Audio => {
                    if original_track
                        .as_any()
                        .downcast_ref::<AudioTrack>()
                        .is_some()
                        && self
                            .proj_ctx
                            .project
                            .get_track(new_track_id)
                            .is_some_and(|t| t.as_any().downcast_ref::<AudioTrack>().is_some())
                        && let Some(original_track) =
                            self.proj_ctx.project.get_track_mut(original_track_id)
                        && let Some(original_audio_track) =
                            original_track.as_any_mut().downcast_mut::<AudioTrack>()
                        && let Some(mut region) = original_audio_track.take_region(region_id)
                    {
                        region.start = new_start;
                        if let Some(new_audio_track) = self
                            .proj_ctx
                            .project
                            .get_track_mut(new_track_id)
                            .and_then(|t| t.as_any_mut().downcast_mut::<AudioTrack>())
                        {
                            new_region_id = Some(new_audio_track.add_region(region));
                        }
                    }
                }
                TrackType::Note => {
                    if original_track
                        .as_any()
                        .downcast_ref::<NoteTrack>()
                        .is_some()
                        && self
                            .proj_ctx
                            .project
                            .get_track(new_track_id)
                            .is_some_and(|t| t.as_any().downcast_ref::<NoteTrack>().is_some())
                        && let Some(original_track) =
                            self.proj_ctx.project.get_track_mut(original_track_id)
                        && let Some(original_note_track) =
                            original_track.as_any_mut().downcast_mut::<NoteTrack>()
                        && let Some(mut region) = original_note_track.take_region(region_id)
                    {
                        region.start = new_start;
                        if let Some(new_note_track) = self
                            .proj_ctx
                            .project
                            .get_track_mut(new_track_id)
                            .and_then(|t| t.as_any_mut().downcast_mut::<NoteTrack>())
                        {
                            new_region_id = Some(new_note_track.add_region(region));
                        }
                    }
                }
            }

            if let Some(new_region_id) = new_region_id
                && self.proj_ctx.project_meta.get_track(new_track_id).is_some()
                && let Some(original_track_meta) =
                    self.proj_ctx.project_meta.get_track_mut(original_track_id)
                // Remove the region from the old track...
                && let Some(mut region_meta) = original_track_meta.remove_region(region_id)
            {
                // ...and move the region in the region meta to the new track
                region_meta.move_region(new_start);
                if let Some(new_track_meta) = self.proj_ctx.project_meta.get_track_mut(new_track_id)
                {
                    new_track_meta.add_region(new_region_id, region_meta);
                    self.ui_state.select_region(*new_track_id, new_region_id);
                }

                // Finally move the calculated waveform data to the new track
                if let Some(waveform) = self
                    .ui_state
                    .timeline_state
                    .waveforms
                    .remove(&(*original_track_id, *region_id))
                {
                    self.ui_state
                        .timeline_state
                        .waveforms
                        .insert((*new_track_id, new_region_id), waveform);
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
