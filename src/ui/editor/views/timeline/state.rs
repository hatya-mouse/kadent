use crate::background_thread::WaveformLod;
use crate::core::audio_engine::{mixer::TrackID, track::RegionID};
use std::{collections::HashMap, path::PathBuf};

#[derive(Default)]
pub(crate) struct TimelineState {
    // --- AUDIO FILE IMPORT ---
    /// The last audio file dropped into the editor, along with the position where it was dropped.
    pub(crate) last_audio_drop: Option<PathBuf>,
    /// Currently being dragged file and its position.
    pub(crate) dragging_audio_file: Option<PathBuf>,
    /// Lastly added region corresponds to an audio file.
    /// This is used to move the region to another track when adding a region by dropping an audio file.
    pub(crate) last_dropped_region: Option<(TrackID, RegionID)>,

    // --- WAVEFORM CACHE ---
    /// Waveform cache for audio regions. The key is a tuple of (track_id, region_id).
    pub(crate) waveforms: HashMap<(TrackID, RegionID), WaveformLod>,
}
