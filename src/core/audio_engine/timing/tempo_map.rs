use crate::core::audio_engine::{
    data_types::Ticks,
    timing::{TempoEvent, TempoSection},
    utils::{samples_to_seconds, seconds_to_samples},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(crate) struct TempoMap {
    /// The tempo events in the tempo map, sorted by their ticks.
    pub(crate) events: Vec<TempoEvent>,
    /// The initial BPM of the tempo map, used before the first tempo event.
    pub(crate) initial_bpm: f64,
    /// The resolution of the audio context, used for converting between ticks and seconds.
    pub(crate) resolution: u64,
}

impl TempoMap {
    // --- NEW ---

    /// Creates a new TempoMap.
    pub(crate) fn new(resolution: u64, initial_bpm: f64) -> Self {
        let mut map = Self {
            events: vec![],
            initial_bpm,
            resolution,
        };
        map.bake();
        map
    }

    // --- AUDIO CONTEXT ---

    /// Sets the resolution to the given one.
    pub(crate) fn set_resolution(&mut self, resolution: u64) {
        self.resolution = resolution;
        self.bake();
    }

    // --- TEMPO EVENT MANAGEMENT ---

    /// Adds a new tempo event to the tempo map, preserving the order of events.
    pub(crate) fn add_event(&mut self, event: TempoEvent) {
        // Insert the event while preserving the order
        match self.events.binary_search(&event) {
            Ok(index) => {
                // Overwrite the old event if the event with the same beat exists
                self.events[index] = event;
                index
            }
            Err(index) => {
                self.events.insert(index, event);
                index
            }
        };

        // Calculate the sample offsets of the events after the modified index
        self.bake();
    }

    /// Removes the tempo event from the tempo map.
    pub(crate) fn remove_event(&mut self, index: usize) {
        // Cannot return the event with the zero beats
        if index == 0 {
            return;
        }

        // Ensure that the TempoMap has at least one event
        if self.events.len() > 1 {
            // Remove the event in the index
            self.events.remove(index);
            // Calculate the sample offsets of the events after the removed index
            self.bake();
        }
    }

    /// Change the BPM of the event in the specified index.
    pub(crate) fn change_bpm(&mut self, index: usize, bpm: f64) {
        // Get a mutable reference to the target event
        let Some(event) = self.events.get_mut(index) else {
            return;
        };

        // Update the bpm
        event.bpm = bpm;

        // Calculate the sample offsets of the events after the event
        self.bake();
    }

    // --- OFFSET CALCULATION ---

    /// Recalculates the seconds for the tempo events.
    fn bake(&mut self) {
        self.events.sort();
        let mut current_time = 0.0;
        let mut prev_tick = 0i64;
        let mut prev_bpm = self.initial_bpm;

        for event in self.events.iter_mut() {
            // Calculate the seconds from the previous event to the current event
            let delta_ticks = event.tick.0.saturating_sub(prev_tick);
            let seconds_per_tick = 60.0 / (prev_bpm * self.resolution as f64);
            current_time += delta_ticks as f64 * seconds_per_tick;

            // Update the event's time_seconds and the previous tick and bpm for the next iteration
            event.time_seconds = current_time;
            prev_tick = event.tick.0;
            prev_bpm = event.bpm;
        }
    }

    // --- BPM GETTING ---

    pub(crate) fn bpm_at_tick(&self, tick: Ticks) -> f64 {
        let idx = self.events.partition_point(|e| e.tick.0 <= tick.0);
        if idx == 0 {
            self.initial_bpm
        } else {
            self.events[idx - 1].bpm
        }
    }

    // --- TICKS CONVERSION ---

    /// Converts the Ticks to seconds using the tempo map.
    pub(crate) fn ticks_to_seconds(&self, tick: Ticks) -> f64 {
        let idx = self.events.partition_point(|e| e.tick.0 <= tick.0);

        if idx == 0 {
            // The tick is before the first event
            let seconds_per_tick = self.seconds_per_tick(self.initial_bpm);
            tick.0 as f64 * seconds_per_tick
        } else {
            // Get the last event before the tick and calculate seconds from the event to the tick
            let event = &self.events[idx - 1];
            let delta_ticks = tick.0 - event.tick.0;
            let seconds_per_tick = self.seconds_per_tick(event.bpm);
            // Then add the calculated seconds to the event's time_seconds
            event.time_seconds + (delta_ticks as f64 * seconds_per_tick)
        }
    }

    /// Converts the seconds to Ticks using the tempo map.
    pub(crate) fn seconds_to_ticks(&self, seconds: f64) -> Ticks {
        let idx = self.events.partition_point(|e| e.time_seconds <= seconds);

        if idx == 0 {
            // The tick is before the first event
            let ticks_per_second = self.ticks_per_second(self.initial_bpm);
            Ticks((seconds * ticks_per_second) as i64)
        } else {
            // Get the last event before the tick and calculate seconds from the event to the tick
            let event = &self.events[idx - 1];
            let delta_seconds = seconds - event.time_seconds;
            let ticks_per_second = self.ticks_per_second(event.bpm);
            // Add the calculated ticks to the event's tick
            Ticks(event.tick.0 + (delta_seconds * ticks_per_second) as i64)
        }
    }

    /// Converts Ticks to samples using the tempo map.
    #[inline]
    pub(crate) fn ticks_to_samples(&self, tick: Ticks, sample_rate: u64) -> usize {
        seconds_to_samples(self.ticks_to_seconds(tick), sample_rate)
    }

    /// Converts samples to Ticks using the tempo map.
    #[inline]
    pub(crate) fn samples_to_ticks(&self, sample: usize, sample_rate: u64) -> Ticks {
        self.seconds_to_ticks(samples_to_seconds(sample, sample_rate))
    }

    /// Calculates seconds per tick for the given bpm.
    #[inline]
    fn seconds_per_tick(&self, bpm: f64) -> f64 {
        60.0 / (bpm * self.resolution as f64)
    }

    /// Calculates ticks per second for the given bpm.
    #[inline]
    fn ticks_per_second(&self, bpm: f64) -> f64 {
        bpm * self.resolution as f64 / 60.0
    }

    // --- SECTION CALCULATION ---

    /// Returns a vector of TempoSection that holds the tempo and the range in it.
    ///
    /// # Parameters
    /// - `start_sample`: The global start sample index of the range.
    /// - `end_sample`: The global end sample index of the range.
    /// - `sample_rate`: The global sample rate.
    pub(crate) fn get_sections_in_range(
        &self,
        start_sample: usize,
        end_sample: usize,
        sample_rate: u64,
    ) -> Vec<TempoSection> {
        let mut sections = Vec::new();
        let sr = sample_rate as f64;
        let inv_sr = 1.0 / sr;

        // If there are no events, create a single section with the initial BPM
        if start_sample >= end_sample || self.events.is_empty() {
            let start_sec = start_sample as f64 * inv_sr;
            let end_sec = end_sample as f64 * inv_sr;
            let start_tick = self.seconds_to_ticks(start_sec);
            let end_tick = self.seconds_to_ticks(end_sec);

            sections.push(TempoSection::new(
                start_tick,
                end_tick - start_tick,
                start_sample,
                end_sample,
                self.initial_bpm,
            ));
            return sections;
        }

        // Calculate where to start iterating over the tempo events
        let start_seconds = samples_to_seconds(start_sample, sample_rate);
        let start_index = self
            .events
            .partition_point(|e| e.time_seconds <= start_seconds);
        let mut current_sample = start_sample;

        // If the start_sample is before the first event, create a section with the initial BPM
        // |      |                           * TempoEvent
        // |      |<-- initial BPM section -->|<--
        // 0      ^ start_sample              ^ section_end
        let first_event_sample = (self.events[0].time_seconds * sr).round() as usize;
        if start_index == 0 && current_sample < first_event_sample {
            let section_end = end_sample.min(first_event_sample);
            if current_sample < section_end {
                let start_sec = current_sample as f64 * inv_sr;
                let end_sec = section_end as f64 * inv_sr;
                let start_tick = self.seconds_to_ticks(start_sec);
                let end_tick = self.seconds_to_ticks(end_sec);

                sections.push(TempoSection::new(
                    start_tick,
                    end_tick - start_tick,
                    current_sample,
                    section_end,
                    self.initial_bpm,
                ));

                // Use the section_end as the new current_sample for the next iteration
                current_sample = section_end;
            }

            if current_sample >= end_sample {
                return sections;
            }
        }

        // Loop over the tempo change events and create sections
        let event_start_idx = if start_index == 0 { 0 } else { start_index - 1 };
        for i in event_start_idx..self.events.len() {
            let bpm = self.events[i].bpm;
            let next_event_sample = self
                .events
                .get(i + 1)
                .map(|e| (e.time_seconds * sample_rate as f64).round() as usize)
                .unwrap_or(usize::MAX);
            let section_end = next_event_sample.min(end_sample);

            if current_sample < section_end {
                let start_sec = current_sample as f64 / sr;
                let end_sec = section_end as f64 / sr;
                let start_tick = self.seconds_to_ticks(start_sec);
                let end_tick = self.seconds_to_ticks(end_sec);

                sections.push(TempoSection::new(
                    start_tick,
                    end_tick - start_tick,
                    current_sample,
                    section_end,
                    bpm,
                ));

                // Use the section_end as the new current_sample for the next iteration
                current_sample = section_end;
            }

            if current_sample >= end_sample {
                break;
            }
        }

        sections
    }
}
