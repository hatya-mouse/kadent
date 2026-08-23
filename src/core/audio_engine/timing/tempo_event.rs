use crate::core::audio_engine::data_types::Ticks;
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub(crate) struct TempoEvent {
    /// The ticks at which the tempo change occurs.
    pub(crate) tick: Ticks,
    /// The bpm which the event represents.
    pub(crate) bpm: f64,
    /// Cached accumulated event position in seconds.
    pub(crate) time_seconds: f64,
}

impl TempoEvent {
    pub(crate) fn new(tick: Ticks, bpm: f64) -> Self {
        Self {
            tick,
            bpm,
            time_seconds: 0.0,
        }
    }
}

impl PartialEq for TempoEvent {
    fn eq(&self, other: &Self) -> bool {
        self.tick == other.tick
    }
}

impl Eq for TempoEvent {}

impl PartialOrd for TempoEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TempoEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.tick > other.tick {
            Ordering::Greater
        } else if self.tick == other.tick {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}
