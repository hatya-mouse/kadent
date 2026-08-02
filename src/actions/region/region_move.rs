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

            match original_track_meta.track_type {
                TrackType::Audio => {
                    // Confirm the destination exists and is the right type *before* removing
                    // the region from the source track, so a failed lookup can never lose data.
                    if original_track
                        .as_any_mut()
                        .downcast_mut::<AudioTrack>()
                        .is_some()
                        && self
                            .proj_ctx
                            .project
                            .get_track_mut(new_track_id)
                            .is_some_and(|t| t.as_any_mut().downcast_mut::<AudioTrack>().is_some())
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
                            new_audio_track.add_region(region);
                        }
                    }
                }
                TrackType::Note => {
                    if original_track
                        .as_any_mut()
                        .downcast_mut::<NoteTrack>()
                        .is_some()
                        && self
                            .proj_ctx
                            .project
                            .get_track_mut(new_track_id)
                            .is_some_and(|t| t.as_any_mut().downcast_mut::<NoteTrack>().is_some())
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
                            new_note_track.add_region(region);
                        }
                    }
                }
            }

            // Also move track in the region meta. Confirm the destination exists *before*
            // removing the region meta from the source, so a failed lookup can never lose data.
            if self
                .proj_ctx
                .project_meta
                .get_track(new_track_id)
                .is_some()
                && let Some(original_track_meta) =
                    self.proj_ctx.project_meta.get_track_mut(original_track_id)
                && let Some(mut region_meta) = original_track_meta.remove_region(region_id)
            {
                region_meta.move_region(new_start);
                if let Some(new_track_meta) =
                    self.proj_ctx.project_meta.get_track_mut(new_track_id)
                {
                    new_track_meta.add_region(*region_id, region_meta);
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
