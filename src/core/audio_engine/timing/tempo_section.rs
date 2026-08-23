use crate::core::audio_engine::data_types::Ticks;

/// Represents a single section which is separated by tempo changes.
#[derive(Debug, Clone)]
pub(crate) struct TempoSection {
    /// The start tick of the section (inclusive).
    pub(crate) start_tick: Ticks,
    /// The duration of the section in ticks.
    pub(crate) duration_tick: Ticks,
    /// The global start sample index of the section (inclusive).
    pub(crate) start_sample: usize,
    /// The global end sample index of the section (exclusive).
    pub(crate) end_sample: usize,
    /// The BPM associated with the section.
    pub(crate) bpm: f64,
}

impl TempoSection {
    pub(crate) fn new(
        start_tick: Ticks,
        duration_tick: Ticks,
        start_sample: usize,
        end_sample: usize,
        bpm: f64,
    ) -> Self {
        Self {
            start_tick,
            duration_tick,
            start_sample,
            end_sample,
            bpm,
        }
    }
}
