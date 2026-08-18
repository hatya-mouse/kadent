use kadent_engine::data_types::Ticks;

#[derive(Default)]
pub(crate) struct TransportState {
    /// Whether the audio is playing.
    pub is_playing: bool,
    /// The current playhead position in ticks.
    pub playhead_tick: Ticks,
}
