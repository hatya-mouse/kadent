use crate::core::audio_engine::data_types::{Beats, Ticks};
use serde::{Deserialize, Serialize};

/// The consistent setting for the project.
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub(crate) struct AudioContext {
    /// Represents how many ticks are in one beat.
    pub(crate) resolution: u64,
}

impl AudioContext {
    pub(crate) fn ticks_to_beats(&self, ticks: Ticks) -> Beats {
        Beats(ticks.0 as f64 / self.resolution as f64)
    }

    pub(crate) fn beats_to_ticks(&self, beats: Beats) -> Ticks {
        Ticks((beats.0 * self.resolution as f64) as i64)
    }
}

/// The playback context for the current playback session.
/// This may change when user changes the output device.
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub(crate) struct PlaybackContext {
    pub(crate) channels: usize,
    pub(crate) sample_rate: u64,
    /// Number of samples in the buffer for each channel.
    pub(crate) buffer_size: usize,
}
