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
