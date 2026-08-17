use crate::background_thread::WaveformLod;
use kadent_engine::{mixer::TrackID, track::RegionID};
use std::{collections::HashMap, path::PathBuf};

pub(crate) struct TimelineState {
    /// The width of the track list.
    pub track_list_width: f32,

    // --- AUDIO FILE IMPORT ---
    /// The last audio file dropped into the editor, along with the position where it was dropped.
    pub last_audio_drop: Option<PathBuf>,
    /// Currently being dragged file and its position.
    pub dragging_audio_file: Option<PathBuf>,
    /// Lastly added region corresponds to an audio file.
    /// This is used to move the region to another track when adding a region by dropping an audio file.
    pub last_dropped_region: Option<(TrackID, RegionID)>,

    // --- WAVEFORM CACHE ---
    /// Waveform cache for audio regions. The key is a tuple of (track_id, region_id).
    pub waveforms: HashMap<(TrackID, RegionID), WaveformLod>,
}

impl Default for TimelineState {
    fn default() -> Self {
        Self {
            track_list_width: 200.0,
            last_audio_drop: None,
            dragging_audio_file: None,
            last_dropped_region: None,
            waveforms: HashMap::new(),
        }
    }
}
