use crate::core::audio_engine::{
    audio_data::AudioSource,
    data_types::Ticks,
    mixer::TrackID,
    timing::TimeBounds,
    track::{
        audio_track::{AudioRegion, AudioTrack},
        note_track::{NoteRegion, NoteTrack},
    },
};
use crate::{
    background_thread::{BackgroundTaskStatus, BackgroundThreadCommand, DecodedAudio},
    core::metadata::{RegionMeta, TrackType},
    ui::{EditorState, theme},
};

impl EditorState {
    /// Adds a new empty audio region to the given audio track.
    pub(crate) fn add_audio_region(
        &mut self,
        track_id: &TrackID,
        name: String,
        bounds: TimeBounds,
    ) {
        // Get the target track
        let Some(track) = self.project.data.get_track_mut(track_id) else {
            return;
        };

        // Cast the track to AudioTrack
        if let Some(audio_track) = track.as_any_mut().downcast_mut::<AudioTrack>() {
            // Create a region and add it to the audio track
            let base_bpm = 120.0;
            let audio_region = AudioRegion::zeros(bounds.clone(), base_bpm);
            let region_id = audio_track.add_region(audio_region);

            // Add a region to the project meta
            if let Some(track_meta) = self.project.meta.get_track_mut(track_id) {
                let region_meta = RegionMeta::new(name, bounds);
                track_meta.add_region(region_id, region_meta);
            }

            // Update the project on the audio thread
            self.modified_project();
        }
    }

    /// Adds a new empty audio region to the given audio track.
    pub(crate) fn add_note_region(&mut self, track_id: &TrackID, name: String, bounds: TimeBounds) {
        // Get the target track
        let Some(track) = self.project.data.get_track_mut(track_id) else {
            return;
        };

        // Cast the track to AudioTrack
        if let Some(audio_track) = track.as_any_mut().downcast_mut::<NoteTrack>() {
            // Create a region and add it to the audio track
            let note_region = NoteRegion::new(bounds.clone());
            let region_id = audio_track.add_region(note_region);

            // Add a region to the project meta
            if let Some(track_meta) = self.project.meta.get_track_mut(track_id) {
                // Note region can be resized as you want
                let region_meta = RegionMeta::new(name, bounds);
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
        // Calculate the length of the audio region to add
        let current_bpm = self.project.data.tempo_map.bpm_at_tick(start);
        let duration_seconds = decoded.frames as f64 / decoded.sample_rate as f64;
        let start_seconds = self.project.data.tempo_map.ticks_to_seconds(start);
        let bounds = TimeBounds::Time {
            start_seconds,
            duration_seconds,
        };

        // Automatically choose the audio track to add the region to
        let region_name = file_name.unwrap_or("Imported File".to_string());
        let track_id = self.available_audio_track(&region_name);

        // Get the audio track
        let Some(track) = self.project.data.get_track_mut(&track_id) else {
            return;
        };

        self.views.status_bar.current_task = None;

        if let Some(audio_track) = track.as_any_mut().downcast_mut::<AudioTrack>() {
            let source = AudioSource::Original(decoded.path);
            let audio_region = AudioRegion::new(source.clone(), bounds.clone(), 0, current_bpm);
            let region_id = audio_track.add_region(audio_region);

            self.views.timeline.last_dropped_region = Some((track_id, region_id));

            // Set the name of the region to the file name or fallback to the default name
            if let Some(track_meta) = self.project.meta.get_track_mut(&track_id) {
                let region_meta = RegionMeta::new(region_name, bounds);
                track_meta.add_region(region_id, region_meta);
            }

            self.modified_project();

            // Send the background thread a message to calculate the waveform of the audio region
            self.views.status_bar.current_task = Some(BackgroundTaskStatus::GenerateWaveform);
            self.push_background_job(BackgroundThreadCommand::GenerateWaveform {
                track_id,
                region_id,
                source,
            });
        }
    }

    fn available_audio_track(&mut self, file_name: &str) -> TrackID {
        if let Some(selected_audio_track) = self.selection.track_id() {
            selected_audio_track
        } else if let Some(first_audio_track) = self
            .project
            .meta
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
