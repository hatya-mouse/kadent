use crate::core::audio_engine::{data_types::Ticks, timing::TempoMap, utils::seconds_to_samples};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Timebase {
    /// Tick / beat based timing
    Musical,
    /// Actual-time based timing
    Time,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub(crate) enum TimePosition {
    Musical(Ticks),
    Time(f64),
}

impl TimePosition {
    /// Returns the timebase of the time bounds.
    pub(crate) fn timebase(&self) -> Timebase {
        match *self {
            TimePosition::Musical(_) => Timebase::Musical,
            TimePosition::Time(_) => Timebase::Time,
        }
    }

    /// Converts the time position to ticks using the given tempo map.
    pub(crate) fn to_ticks(&self, tempo_map: &TempoMap) -> Ticks {
        match *self {
            TimePosition::Musical(ticks) => ticks,
            TimePosition::Time(seconds) => tempo_map.seconds_to_ticks(seconds),
        }
    }

    /// Converts the time position to seconds using the given tempo map.
    pub(crate) fn to_seconds(&self, tempo_map: &TempoMap) -> f64 {
        match *self {
            TimePosition::Musical(ticks) => tempo_map.ticks_to_seconds(ticks),
            TimePosition::Time(seconds) => seconds,
        }
    }

    /// Converts the time position to seconds using the given tempo map.
    pub(crate) fn to_sample(&self, tempo_map: &TempoMap, sample_rate: u64) -> usize {
        match *self {
            TimePosition::Musical(ticks) => tempo_map.ticks_to_samples(ticks, sample_rate),
            TimePosition::Time(seconds) => seconds_to_samples(seconds, sample_rate),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum TimeBounds {
    Musical {
        start: Ticks,
        duration: Ticks,
    },
    Time {
        start_seconds: f64,
        duration_seconds: f64,
    },
}

impl TimeBounds {
    pub(crate) const ZERO: TimeBounds = TimeBounds::Musical {
        start: Ticks(0),
        duration: Ticks(0),
    };

    /// Returns the timebase of the time bounds.
    pub(crate) fn timebase(&self) -> Timebase {
        match *self {
            TimeBounds::Musical { .. } => Timebase::Musical,
            TimeBounds::Time { .. } => Timebase::Time,
        }
    }

    // --- TIMEPOSITION GETTING ---

    /// Returns the start position.
    pub(crate) fn start_time(&self) -> TimePosition {
        match *self {
            TimeBounds::Musical { start, .. } => TimePosition::Musical(start),
            TimeBounds::Time { start_seconds, .. } => TimePosition::Time(start_seconds),
        }
    }

    /// Return the end position.
    pub(crate) fn end_time(&self) -> TimePosition {
        match *self {
            TimeBounds::Musical { start, duration } => TimePosition::Musical(start + duration),
            TimeBounds::Time {
                start_seconds,
                duration_seconds,
            } => TimePosition::Time(start_seconds + duration_seconds),
        }
    }

    // --- TICK CALCULATION ---

    /// Calculates the start and end ticks from the given tempo map.
    pub(crate) fn tick_range(&self, tempo_map: &TempoMap) -> (Ticks, Ticks) {
        (self.start_tick(tempo_map), self.end_tick(tempo_map))
    }

    /// Calculates the start tick from the given tempo map.
    pub(crate) fn start_tick(&self, tempo_map: &TempoMap) -> Ticks {
        match *self {
            TimeBounds::Musical { start, .. } => start,
            TimeBounds::Time { start_seconds, .. } => tempo_map.seconds_to_ticks(start_seconds),
        }
    }

    /// Calculates the end tick from the given tempo map.
    pub(crate) fn end_tick(&self, tempo_map: &TempoMap) -> Ticks {
        match *self {
            TimeBounds::Musical { start, duration } => start + duration,
            TimeBounds::Time {
                start_seconds,
                duration_seconds,
            } => tempo_map.seconds_to_ticks(start_seconds + duration_seconds),
        }
    }

    /// Calculates the duration of the time bounds in ticks from the given tempo map.
    pub(crate) fn duration_ticks(&self, tempo_map: &TempoMap) -> Ticks {
        match *self {
            TimeBounds::Musical { duration, .. } => duration,
            TimeBounds::Time {
                start_seconds,
                duration_seconds,
            } => {
                let start_ticks = tempo_map.seconds_to_ticks(start_seconds);
                let end_ticks = tempo_map.seconds_to_ticks(start_seconds + duration_seconds);
                end_ticks - start_ticks
            }
        }
    }

    // --- SECONDS CALCULATION ---

    /// Calculates the start seconds from the given tempo map.
    pub(crate) fn start_seconds(&self, tempo_map: &TempoMap) -> f64 {
        match *self {
            TimeBounds::Musical { start, .. } => tempo_map.ticks_to_seconds(start),
            TimeBounds::Time { start_seconds, .. } => start_seconds,
        }
    }

    /// Calculates the end tick from the given tempo map.
    pub(crate) fn end_seconds(&self, tempo_map: &TempoMap) -> f64 {
        match *self {
            TimeBounds::Musical { start, duration } => tempo_map.ticks_to_seconds(start + duration),
            TimeBounds::Time {
                start_seconds,
                duration_seconds,
            } => start_seconds + duration_seconds,
        }
    }

    /// Calculates the duration of the time bounds in seconds from the given tempo map.
    pub(crate) fn duration_seconds(&self, tempo_map: &TempoMap) -> f64 {
        match *self {
            TimeBounds::Musical { start, duration } => {
                let start_seconds = tempo_map.ticks_to_seconds(start);
                let end_seconds = tempo_map.ticks_to_seconds(start + duration);
                end_seconds - start_seconds
            }
            TimeBounds::Time {
                duration_seconds, ..
            } => duration_seconds,
        }
    }

    // --- SAMPLE CONVERSION ---

    // Calculate the start and end samples from the given tempo map and sample rate.
    pub(crate) fn sample_range(&self, tempo_map: &TempoMap, sample_rate: u64) -> (usize, usize) {
        (
            self.start_sample(tempo_map, sample_rate),
            self.end_sample(tempo_map, sample_rate),
        )
    }

    // Calculates the start sample from the given tempo map and sample rate.
    pub(crate) fn start_sample(&self, tempo_map: &TempoMap, sample_rate: u64) -> usize {
        let start_seconds = self.start_seconds(tempo_map);
        seconds_to_samples(start_seconds, sample_rate)
    }

    // Calculates the end sample from the given tempo map and sample rate.
    pub(crate) fn end_sample(&self, tempo_map: &TempoMap, sample_rate: u64) -> usize {
        let end_seconds = self.end_seconds(tempo_map);
        seconds_to_samples(end_seconds, sample_rate)
    }

    // --- TICKS SETTER ---

    /// Sets the start tick of the time bounds.
    pub(crate) fn set_start_tick(&mut self, tick: Ticks, tempo_map: &TempoMap) {
        match self {
            TimeBounds::Musical { start, .. } => *start = tick,
            TimeBounds::Time { start_seconds, .. } => {
                *start_seconds = tempo_map.ticks_to_seconds(tick)
            }
        }
    }

    /// Sets the duration of the time bounds.
    pub(crate) fn set_duration_ticks(&mut self, duration: Ticks, tempo_map: &TempoMap) {
        match self {
            TimeBounds::Musical { duration: dur, .. } => *dur = duration,
            TimeBounds::Time {
                duration_seconds,
                start_seconds,
            } => {
                let start_tick = tempo_map.seconds_to_ticks(*start_seconds);
                let end_seconds = tempo_map.ticks_to_seconds(start_tick + duration);
                *duration_seconds = end_seconds - *start_seconds;
            }
        }
    }
}
