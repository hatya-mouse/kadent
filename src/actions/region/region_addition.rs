use crate::{
    background_thread::DecodedAudio,
    core::metadata::{RegionMeta, TrackType},
    ui::{theme, workspaces::EditorUi},
};
use kadent_engine::{
    data_types::Ticks,
    mixer::TrackID,
    track::{
        RegionID,
        audio_track::{AudioRegion, AudioTrack},
        note_track::{NoteRegion, NoteTrack},
    },
};
use std::collections::HashMap;

impl EditorUi {
    /// Adds a new empty audio region to the given audio track.
    pub(in crate::actions) fn add_audio_region(
        &mut self,
        track_id: &TrackID,
        name: String,
        start: Ticks,
    ) {
        let sample_rate = self.ui_state.audio_ctx.sample_rate;
        let channels = self.ui_state.audio_ctx.channels;
        let duration = Ticks(self.ui_state.audio_ctx.resolution as i64);

        // Get the target track
        let Some(track) = self.proj_ctx.project.get_track_mut(track_id) else {
            return;
        };

        // Cast the track to AudioTrack
        if let Some(audio_track) = track.as_any_mut().downcast_mut::<AudioTrack>() {
            // Create a region and add it to the audio track
            let base_bpm = 120.0;
            let frames = (duration.0 as f64
                / (self.ui_state.audio_ctx.resolution as f64 * base_bpm))
                as usize
                * 60
                * sample_rate as usize;
            let audio_region = AudioRegion::zeros(
                frames,
                sample_rate as u32,
                channels as u16,
                base_bpm,
                start,
                duration,
            );
            let max_duration = audio_region.max_duration;
            let region_id = audio_track.add_region(audio_region);

            // Add a region to the project meta
            if let Some(track_meta) = self.proj_ctx.project_meta.get_track_mut(track_id) {
                let region_meta = RegionMeta::new(name, start, duration, Some(max_duration));
                track_meta.add_region(region_id, region_meta);
            }

            // Update the project on the audio thread
            self.modified_project();
        }
    }

    /// Adds a new empty audio region to the given audio track.
    pub(in crate::actions) fn add_note_region(
        &mut self,
        track_id: &TrackID,
        name: String,
        start: Ticks,
    ) {
        let duration = Ticks(self.ui_state.audio_ctx.resolution as i64);

        // Get the target track
        let Some(track) = self.proj_ctx.project.get_track_mut(track_id) else {
            return;
        };

        // Cast the track to AudioTrack
        if let Some(audio_track) = track.as_any_mut().downcast_mut::<NoteTrack>() {
            // Create a region and add it to the audio track
            let note_region = NoteRegion::new(start, duration);
            let region_id = audio_track.add_region(note_region);

            // Add a region to the project meta
            if let Some(track_meta) = self.proj_ctx.project_meta.get_track_mut(track_id) {
                // Note region can be resized as you want
                let region_meta = RegionMeta::new(name, start, duration, None);
                track_meta.add_region(region_id, region_meta);
            }

            // Update the project on the audio thread
            self.modified_project();
        }
    }

    pub(crate) fn finish_audio_import(
        &mut self,
        file_name: Option<String>,
        start: Ticks,
        decoded: DecodedAudio,
    ) {
        let min_duration = Ticks(self.ui_state.audio_ctx.resolution as i64);

        // Calculate the length of the audio region to add
        let current_bpm = {
            let events = &self.proj_ctx.project.tempo_map.events;
            let idx = events
                .partition_point(|e| e.ticks() <= start)
                .saturating_sub(1);
            events[idx].bpm()
        };
        let duration_seconds = decoded.frames as f64 / decoded.sample_rate as f64;
        let data_duration = Ticks(
            (duration_seconds / 60.0 * current_bpm * self.ui_state.audio_ctx.resolution as f64)
                .round() as i64,
        );

        // Automatically choose the audio track to add the region to
        let region_name = file_name.unwrap_or("Imported File".to_string());
        let track_id = self.available_audio_track(&region_name);

        // Get the audio track
        let Some(track) = self.proj_ctx.project.get_track_mut(&track_id) else {
            return;
        };
        // Then calculate the duration of the region to prevent region overlapping
        let Some(region_duration) = self
            .proj_ctx
            .project_meta
            .get_track(&track_id)
            .map(|t| &t.regions)
            .and_then(|regions| {
                calculate_region_placement(regions, start, data_duration, Some(min_duration))
            })
        else {
            return;
        };

        self.ui_state.status_bar_state.current_task = None;

        if let Some(audio_track) = track.as_any_mut().downcast_mut::<AudioTrack>() {
            let audio_region = AudioRegion {
                data: decoded.data,
                frames: decoded.frames,
                sample_rate: decoded.sample_rate,
                channels: decoded.channels,
                base_bpm: current_bpm,
                start,
                duration: region_duration,
                max_duration: data_duration,
            };
            let region_id = audio_track.add_region(audio_region);

            self.ui_state.timeline_state.last_dropped_region = Some((track_id, region_id));

            // Set the name of the region to the file name or fallback to the default name
            if let Some(track_meta) = self.proj_ctx.project_meta.get_track_mut(&track_id) {
                let region_meta =
                    RegionMeta::new(region_name, start, region_duration, Some(data_duration));
                track_meta.add_region(region_id, region_meta);
            }

            self.modified_project();
        }
    }

    fn available_audio_track(&mut self, file_name: &str) -> TrackID {
        if let Some(selected_audio_track) = self.ui_state.selection.track_id() {
            selected_audio_track
        } else if let Some(first_audio_track) = self
            .proj_ctx
            .project_meta
            .tracks
            .iter()
            .find(|track| matches!(track.1.track_type, TrackType::Audio))
            .map(|track| *track.0)
        {
            first_audio_track
        } else {
            self.add_track(
                TrackType::Audio,
                file_name.to_string(),
                theme::selected_bg(),
            )
        }
    }
}

/// Returns None if start is already occupied, or Some(clamped_duration) otherwise.
fn calculate_region_placement(
    regions: &HashMap<RegionID, RegionMeta>,
    start: Ticks,
    desired_duration: Ticks,
    min_duration: Option<Ticks>,
) -> Option<Ticks> {
    // Do not add region if the start beat has already been occupied
    let start_blocked = regions
        .values()
        .any(|r| start.0 >= r.start.0 && start.0 < r.start.0 + r.duration.0);
    if start_blocked {
        return None;
    }

    // Clamp the region using the nearest region
    let clamped_duration = regions
        .values()
        .filter(|r| r.start > start)
        .map(|r| r.start - start)
        .filter(|d| d < &desired_duration)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(desired_duration);

    Some(clamped_duration.max(min_duration.unwrap_or(Ticks(1))))
}
