use crate::core::audio_engine::{
    audio_data::AudioFilePool,
    data_types::{AudioContext, PlaybackContext},
    mixer::{Mixer, track_id::TrackID},
    timing::{TempoMap, TimeBounds},
    track::{Track, error::TrackError},
};
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct ProjectData {
    // --- TRACKS ---
    /// Tracks in the project.
    pub(crate) tracks: HashMap<TrackID, Box<dyn Track>>,

    // --- TEMPO MAP ---
    /// A tempo map to store the tempo changes.
    pub(crate) tempo_map: TempoMap,

    // --- AUDIO CONTEXT ---
    /// An audio context for the project, which stores some configurations.
    pub(crate) audio_ctx: AudioContext,

    // --- RANGE ---
    /// The export range of the project.
    pub(crate) export_range: TimeBounds,

    // --- MISCS ---
    /// The next track ID for generating track IDs.
    next_track_id: u64,
}

impl ProjectData {
    // --- NEW ---

    /// Creates a new project with the specified initial bpm.
    pub(crate) fn new(audio_ctx: AudioContext, bpm: f64, export_range: TimeBounds) -> Self {
        Self {
            tracks: HashMap::new(),
            tempo_map: TempoMap::new(audio_ctx.resolution, bpm),
            audio_ctx,
            export_range,
            next_track_id: 0,
        }
    }

    /// Creates a new project with the given tempo map.
    pub(crate) fn with_tempo_map(
        audio_ctx: AudioContext,
        tempo_map: TempoMap,
        export_range: TimeBounds,
    ) -> Self {
        Self {
            tracks: HashMap::new(),
            tempo_map,
            audio_ctx,
            export_range,
            next_track_id: 0,
        }
    }

    /// Creates a new project with all the specified parameters.
    pub(crate) fn with_all(
        tracks: HashMap<TrackID, Box<dyn Track>>,
        tempo_map: TempoMap,
        audio_ctx: AudioContext,
        export_range: TimeBounds,
        next_track_id: u64,
    ) -> Self {
        Self {
            tracks,
            tempo_map,
            audio_ctx,
            export_range,
            next_track_id,
        }
    }

    // --- TRACK ID GENERATION ---

    /// Sets the next track ID for generating track IDs.
    pub(crate) fn set_next_track_id(&mut self, next_id: u64) {
        self.next_track_id = next_id;
    }

    /// Generates a new unique track ID.
    fn generate_track_id(&mut self) -> TrackID {
        let id = TrackID(self.next_track_id);
        self.next_track_id += 1;
        id
    }

    // --- TRACK MANAGEMENT ---

    /// Adds a new track to the mixer, setting the audio context to the one in the mixer.
    pub(crate) fn add_track(&mut self, mut track: Box<dyn Track>) -> TrackID {
        let id = self.generate_track_id();
        track.update_type_info();
        self.tracks.insert(id, track);
        id
    }

    /// Removes the track from the mixer.
    pub(crate) fn remove_track(&mut self, id: &TrackID) {
        self.tracks.remove(id);
    }

    /// Returns a reference to the track.
    pub(crate) fn get_track(&self, id: &TrackID) -> Option<&dyn Track> {
        self.tracks.get(id).map(|track| &**track)
    }

    /// Returns a mutable reference to the track.
    pub(crate) fn get_track_mut(&mut self, id: &TrackID) -> Option<&mut Box<dyn Track>> {
        self.tracks.get_mut(id)
    }

    // --- MIXING PREPARATION ---

    /// Prepares the tracks in the mixer for the playback.
    /// Tracks that fail to prepare will just be skipped, and their errors will be returned.
    pub(crate) fn prepare(
        mut self,
        audio_pool: &mut AudioFilePool,
        playback_ctx: PlaybackContext,
    ) -> (Mixer, Vec<(TrackID, TrackError)>) {
        // Prepare the tracks one by one, collecting errors instead of aborting on the first one
        let mut errors = Vec::new();
        for (id, track) in self.tracks.iter_mut() {
            if let Err(err) =
                track.prepare(audio_pool, &self.tempo_map, &self.audio_ctx, &playback_ctx)
            {
                errors.push((*id, err));
            }
        }

        (Mixer::new(self, playback_ctx), errors)
    }
}
