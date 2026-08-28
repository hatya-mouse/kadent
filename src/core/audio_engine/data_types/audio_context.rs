/// The consistent setting for the project.
#[derive(Clone, Default, Debug)]
pub(crate) struct AudioContext {
    /// Represents how many ticks are in one beat.
    pub(crate) resolution: u64,
}

/// The playback context for the current playback session.
/// This may change when user changes the output device.
#[derive(Clone, Default, Debug)]
pub(crate) struct PlaybackContext {
    pub(crate) channels: usize,
    pub(crate) sample_rate: u64,
    /// Number of samples in the buffer for each channel.
    pub(crate) buffer_size: usize,
}

impl PlaybackContext {
    pub(in crate::core::audio_engine) fn from_stream_config(
        config: &cpal::StreamConfig,
        fallback_buffer_size: usize,
    ) -> Self {
        PlaybackContext {
            sample_rate: config.sample_rate as u64,
            channels: config.channels as usize,
            buffer_size: match config.buffer_size {
                cpal::BufferSize::Fixed(size) => size as usize,
                cpal::BufferSize::Default => fallback_buffer_size,
            },
        }
    }
}
