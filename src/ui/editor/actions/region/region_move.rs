use crate::{
    core::audio_engine::{
        mixer::TrackID,
        timing::{TimeBounds, TimePosition, Timebase},
        track::{RegionID, audio_track::AudioTrack, note_track::NoteTrack},
    },
    ui::editor::EditorUi,
};
use crate::{core::metadata::TrackType, ui::theme};

impl EditorUi {
    pub(crate) fn move_region(
        &mut self,
        original_track_id: &TrackID,
        region_id: &RegionID,
        new_track_id: &TrackID,
        new_start: TimePosition,
    ) {
        // Move the region to the new start beats
        let Some(original_track) = self.state.project.data.get_track(original_track_id) else {
            return;
        };
        let Some(original_bounds) = original_track.get_region_bounds(region_id) else {
            return;
        };
        let new_bounds = self.create_bounds_from(original_bounds, new_start);

        let Some(original_track) = self.state.project.data.get_track_mut(original_track_id) else {
            return;
        };

        if original_track_id != new_track_id {
            // Move the region to a new track
            let Some(original_track_meta) = self.state.project.meta.get_track(original_track_id)
            else {
                return;
            };
            let Some(new_track_meta) = self.state.project.meta.get_track(new_track_id) else {
                return;
            };

            // Check if the destination track is of the same type as the original track
            if original_track_meta.track_type != new_track_meta.track_type {
                self.views.status_bar.show_temp_status(
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
                            .state
                            .project
                            .data
                            .get_track(new_track_id)
                            .is_some_and(|t| t.as_any().downcast_ref::<AudioTrack>().is_some())
                        && let Some(original_track) =
                            self.state.project.data.get_track_mut(original_track_id)
                        && let Some(original_audio_track) =
                            original_track.as_any_mut().downcast_mut::<AudioTrack>()
                        && let Some(mut region) = original_audio_track.take_region(region_id)
                    {
                        region.bounds = new_bounds.clone();
                        if let Some(new_audio_track) = self
                            .state
                            .project
                            .data
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
                            .state
                            .project
                            .data
                            .get_track(new_track_id)
                            .is_some_and(|t| t.as_any().downcast_ref::<NoteTrack>().is_some())
                        && let Some(original_track) =
                            self.state.project.data.get_track_mut(original_track_id)
                        && let Some(original_note_track) =
                            original_track.as_any_mut().downcast_mut::<NoteTrack>()
                        && let Some(mut region) = original_note_track.take_region(region_id)
                    {
                        region.bounds = new_bounds.clone();
                        if let Some(new_note_track) = self
                            .state
                            .project
                            .data
                            .get_track_mut(new_track_id)
                            .and_then(|t| t.as_any_mut().downcast_mut::<NoteTrack>())
                        {
                            new_region_id = Some(new_note_track.add_region(region));
                        }
                    }
                }
            }

            if let Some(new_region_id) = new_region_id
                && self.state.project.meta.get_track(new_track_id).is_some()
                && let Some(original_track_meta) =
                    self.state.project.meta.get_track_mut(original_track_id)
                // Remove the region from the old track...
                && let Some(mut region_meta) = original_track_meta.remove_region(region_id)
            {
                // ...and move the region in the region meta to the new track
                region_meta.bounds = new_bounds;
                if let Some(new_track_meta) = self.state.project.meta.get_track_mut(new_track_id) {
                    new_track_meta.add_region(new_region_id, region_meta);
                    self.state
                        .selection
                        .select_region(*new_track_id, new_region_id);
                }

                // Finally move the calculated waveform data to the new track
                if let Some(waveform) = self
                    .views
                    .timeline
                    .waveforms
                    .remove(&(*original_track_id, *region_id))
                {
                    self.views
                        .timeline
                        .waveforms
                        .insert((*new_track_id, new_region_id), waveform);
                }
            }
        } else {
            let Some(original_track_meta) =
                self.state.project.meta.get_track_mut(original_track_id)
            else {
                return;
            };

            original_track.set_region_bounds(region_id, new_bounds.clone());

            // Move the region in the region meta too
            if let Some(region_meta) = original_track_meta.get_region_mut(region_id) {
                region_meta.bounds = new_bounds;
            }
        }

        self.state.actions.modified_project();
    }

    pub(crate) fn set_region_duration(
        &mut self,
        track_id: &TrackID,
        region_id: &RegionID,
        new_duration: TimePosition,
    ) {
        let Some(original_bounds) = self
            .state
            .project
            .data
            .get_track(track_id)
            .and_then(|t| t.get_region_bounds(region_id))
        else {
            return;
        };

        let tempo_map = &self.state.project.data.tempo_map;
        let new_bounds = match original_bounds.timebase() {
            Timebase::Musical => TimeBounds::Musical {
                start: original_bounds.start_tick(tempo_map),
                duration: new_duration.to_ticks(tempo_map),
            },
            Timebase::Time => TimeBounds::Time {
                start_seconds: original_bounds.start_seconds(tempo_map),
                duration_seconds: new_duration.to_seconds(tempo_map),
            },
        };

        // Move the region to the new start beats
        if let Some(track) = self.state.project.data.get_track_mut(track_id) {
            track.set_region_bounds(region_id, new_bounds.clone());
        }

        // Set the region start beats in metadata
        if let Some(track_meta) = self.state.project.meta.get_track_mut(track_id)
            && let Some(region_meta) = track_meta.get_region_mut(region_id)
        {
            region_meta.bounds = new_bounds;
        }

        self.state.actions.modified_project();
    }

    fn create_bounds_from(
        &self,
        original_bounds: &TimeBounds,
        new_start: TimePosition,
    ) -> TimeBounds {
        let tempo_map = &self.state.project.data.tempo_map;
        match original_bounds.timebase() {
            Timebase::Musical => TimeBounds::Musical {
                start: new_start.to_ticks(tempo_map),
                duration: original_bounds.duration_ticks(tempo_map),
            },
            Timebase::Time => TimeBounds::Time {
                start_seconds: new_start.to_seconds(tempo_map),
                duration_seconds: original_bounds.duration_seconds(tempo_map),
            },
        }
    }
}
