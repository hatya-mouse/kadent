use crate::core::audio_engine::data_types::Ticks;

#[derive(Default)]
pub(crate) struct TransportState {
    /// Whether the audio is playing.
    pub(crate) is_playing: bool,
    /// The current playhead position in ticks.
    pub(crate) playhead_tick: Ticks,
}
