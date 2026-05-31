use std::collections::HashMap;

use crate::{metadata::RegionMeta, ui::EditorUi};
use kreniq_engine::{
    data_types::Beats,
    mixer::TrackID,
    track::{
        RegionID,
        audio_track::{AudioRegion, AudioTrack},
        note_track::{NoteRegion, NoteTrack},
    },
};

impl EditorUi {
    /// Adds a new empty audio region to the given audio track.
    pub(crate) fn add_audio_region(&mut self, track_id: &TrackID, name: String, start: Beats) {
        let sample_rate = self.project.audio_ctx.sample_rate;
        let channels = self.project.audio_ctx.channels;
        let duration = Beats(1.0);

        // Get the target track
        let Some(track) = self.project.get_track_mut(track_id) else {
            return;
        };
        let Some(duration) = &self
            .project_meta
            .get_track(track_id)
            .map(|track| &track.regions)
            .and_then(|regions| calculate_region_placement(regions, start, duration))
        else {
            return;
        };

        // Cast the track to AudioTrack
        if let Some(audio_track) = track.as_any_mut().downcast_mut::<AudioTrack>() {
            // Create a region and add it to the audio track
            let base_bpm = 120.0;
            let frames = (duration.0 / base_bpm) as usize * 60 * sample_rate;
            let audio_region = AudioRegion::zeros(
                frames,
                sample_rate as u32,
                channels as u16,
                base_bpm,
                start,
                *duration,
            );
            let max_duration = audio_region.max_duration;
            let region_id = audio_track.add_region(audio_region);

            // Add a region to the project meta
            if let Some(track_meta) = self.project_meta.get_track_mut(track_id) {
                let region_meta = RegionMeta::new(name, start, *duration, Some(max_duration));
                track_meta.add_region(region_id, region_meta);
            }

            // Update the project on the audio thread
            self.modified_project();
        }
    }

    /// Adds a new empty audio region to the given audio track.
    pub(crate) fn add_note_region(&mut self, track_id: &TrackID, name: String, start: Beats) {
        let duration = Beats(1.0);

        // Get the target track
        let Some(track) = self.project.get_track_mut(track_id) else {
            return;
        };
        let Some(duration) = &self
            .project_meta
            .get_track(track_id)
            .map(|track| &track.regions)
            .and_then(|regions| calculate_region_placement(regions, start, duration))
        else {
            return;
        };

        // Cast the track to AudioTrack
        if let Some(audio_track) = track.as_any_mut().downcast_mut::<NoteTrack>() {
            // Create a region and add it to the audio track
            let note_region = NoteRegion::new(start, *duration);
            let region_id = audio_track.add_region(note_region);

            // Add a region to the project meta
            if let Some(track_meta) = self.project_meta.get_track_mut(track_id) {
                // Note region can be resized as you want
                let region_meta = RegionMeta::new(name, start, *duration, None);
                track_meta.add_region(region_id, region_meta);
            }

            // Update the project on the audio thread
            self.modified_project();
        }
    }
}

/// Returns None if start is already occupied, or Some(clamped_duration) otherwise.
fn calculate_region_placement(
    regions: &HashMap<RegionID, RegionMeta>,
    start: Beats,
    desired_duration: Beats,
) -> Option<Beats> {
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
        .filter(|r| r.start.0 > start.0)
        .map(|r| Beats(r.start.0 - start.0))
        .filter(|d| d.0 < desired_duration.0)
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .unwrap_or(desired_duration);

    Some(clamped_duration)
}
