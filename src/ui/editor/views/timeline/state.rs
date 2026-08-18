use crate::{background_thread::WaveformLod, ui::editor::TimelineCoord};
use kadent_engine::{mixer::TrackID, track::RegionID};
use std::{collections::HashMap, path::PathBuf};
use uuid::Uuid;

#[derive(Default)]
pub(crate) struct TimelineState {
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

    // --- TIMELINE COORDINATES ---
    /// Timeline coordinates of the panel.
    pub timeline_coords: HashMap<Uuid, TimelineCoord>,
}

impl TimelineState {
    pub(crate) fn remove_panel_state(&mut self, panel_id: &Uuid) {
        self.timeline_coords.remove(panel_id);
    }
}
