use crate::core::audio_engine::{
    MAX_CHANNELS,
    audio_data::{AudioData, AudioFilePool, AudioSource},
    data_types::{AudioContext, PlaybackContext},
    timing::{TempoMap, TimeBounds},
    track::audio_track::resampler::resample_channels,
    utils::{convert_rate_with_ratio, samples_per_tick},
};
use std::sync::Arc;

/// Stores the raw audio source data.
#[derive(Debug, Clone)]
pub(crate) struct AudioRegion {
    pub(crate) data_source: AudioSource,
    /// The bounds for the region.
    pub(crate) bounds: TimeBounds,
    /// The sample index to start reading the audio data from.
    pub(crate) data_offset: usize,
    /// The bpm associated with the audio data. This is used to change the tempo when the tempo of the project is different.
    pub(crate) bpm: f64,

    /// Cached region start and end sample positions in the global sample rate.
    sample_bounds: (usize, usize),
    /// Cached samples per tick for the region, used for sample number calculation in Musical mode.
    data_samples_per_tick: f64,
    /// Cached raw audio data for the region.
    audio_data: Option<Arc<AudioData>>,
    /// Pre-allocated buffer for resampled audio data.
    resampled_buffer: Vec<f32>,
}

impl AudioRegion {
    // --- INITIALIZER ---

    pub(crate) fn new(
        data_source: AudioSource,
        bounds: TimeBounds,
        data_offset: usize,
        bpm: f64,
    ) -> Self {
        Self {
            data_source,
            bounds,
            data_offset,
            bpm,
            sample_bounds: (0, 0),
            data_samples_per_tick: 0.0,
            audio_data: None,
            resampled_buffer: Vec::new(),
        }
    }

    pub(crate) fn zeros(bounds: TimeBounds, bpm: f64) -> Self {
        let data_source = AudioSource::Zero;
        Self {
            data_source,
            bounds,
            data_offset: 0,
            bpm,
            sample_bounds: (0, 0),
            data_samples_per_tick: 0.0,
            audio_data: None,
            resampled_buffer: Vec::new(),
        }
    }

    // --- REGION PROCESSING ---

    pub(super) fn prepare(
        &mut self,
        audio_pool: &mut AudioFilePool,
        tempo_map: &TempoMap,
        audio_ctx: &AudioContext,
        playback_ctx: &PlaybackContext,
    ) {
        self.sample_bounds = self
            .bounds
            .sample_range(tempo_map, playback_ctx.sample_rate);

        // Load the audio data from the audio pool
        self.audio_data = audio_pool.get_or_load(&self.data_source);

        if let Some(audio_data) = &self.audio_data {
            self.resampled_buffer
                .reserve(playback_ctx.buffer_size * audio_data.info.channels);
            self.data_samples_per_tick =
                samples_per_tick(audio_data.info.sample_rate, self.bpm, audio_ctx.resolution);
        } else {
            self.resampled_buffer.clear();
        }
    }

    /// Reads and writes the audio data to the given buffer based on the current playhead position and the buffer size.
    ///
    /// # Parameters
    /// - `playhead`: The current playhead position in the sample rate of current playback context.
    /// - `buffer`: The buffer to write the audio data into.
    /// - `tempo_map`: The tempo map for the current playback context.
    /// - `playback_ctx`: The playback context containing the sample rate and other playback settings.
    pub(super) fn render_buffer(
        &mut self,
        playhead: usize,
        buffer: &mut [f32],
        tempo_map: &TempoMap,
        playback_ctx: &PlaybackContext,
    ) {
        // Skip processing if the audio data is not loaded
        let Some(audio_data) = &self.audio_data else {
            return;
        };

        // Skip processing if the region is entirely outside the buffer
        let buffer_end = playhead + playback_ctx.buffer_size;
        if buffer_end <= self.sample_bounds.0 || playhead >= self.sample_bounds.1 {
            return;
        }

        let data_channels = audio_data.info.channels;
        let data_sample_rate = audio_data.info.sample_rate;
        let sample_rate_ratio = data_sample_rate as f64 / playback_ctx.sample_rate as f64;

        // Calculate where to start and end reading from the audio data
        // This is to handle edge cases like below:
        // Buffer: |<-- Current buffer block -->|
        //         ^ playhead                   ^ buffer_end
        // Region:      [<-- Region -->]
        // global_start ^              ^ global_end
        let global_start = playhead.max(self.sample_bounds.0);
        let global_end = buffer_end.min(self.sample_bounds.1);

        // Calculate the index in the buffer to start writing the audio data to
        // This is used to pad the audio data with zeros if the region starts after the playhead position
        // |<-- Current buffer block...
        // |                     [<-- Region...
        // |<-- start_padding -->|
        // |0, 0, 0, 0, 0, 0, 0, |<-- start writing audio data here
        let start_padding = self.sample_bounds.0.saturating_sub(playhead);
        let mut current_write_index = start_padding * MAX_CHANNELS;

        let sections =
            tempo_map.get_sections_in_range(global_start, global_end, playback_ctx.sample_rate);
        for section in sections {
            let rendered_frames = section.end_sample.saturating_sub(section.start_sample);
            if rendered_frames == 0 {
                continue;
            }

            // Calculate the number of ticks in the current section
            let (data_start, data_end) = match self.bounds {
                TimeBounds::Musical { start, .. } => {
                    // Calculate the start and end sample positions in the audio data for the current section
                    // Tempo Map:                                                                        *<-- Tempo Event
                    // Ticks Space:       0<----------- start ---------->|<-- ticks_from_region_start -->|
                    //                    |<-------------------- section.start_tick -------------------->|<-- section.duration_tick -->|
                    // Region:            |                              [<-- Region...
                    // Section:           |                              |<----- Previous Section ------>|<----- Current Section ----->|
                    // Data Sample Space: |           |<------------------ data_start ------------------>|<-- data_duration_samples -->|
                    //                    |           |<---------------------------------- data_end ---------------------------------->|
                    let ticks_from_region_start = section.start_tick - start;
                    let data_samples_from_region_start =
                        (ticks_from_region_start.0 as f64 * self.data_samples_per_tick) as usize;
                    let data_start_sample = self.data_offset + data_samples_from_region_start;
                    let data_duration_samples =
                        (section.duration_tick.0 as f64 * self.data_samples_per_tick) as usize;
                    let data_end_sample = data_start_sample + data_duration_samples;

                    (
                        data_start_sample * data_channels,
                        data_end_sample * data_channels,
                    )
                }
                TimeBounds::Time { .. } => {
                    let samples_from_region_start =
                        section.start_sample.saturating_sub(self.sample_bounds.0);
                    let data_start_sample = self.data_offset
                        + convert_rate_with_ratio(samples_from_region_start, sample_rate_ratio);

                    let data_section_samples =
                        convert_rate_with_ratio(rendered_frames, sample_rate_ratio);
                    let data_end_sample = data_start_sample + data_section_samples;

                    (
                        data_start_sample * data_channels,
                        data_end_sample * data_channels,
                    )
                }
            };

            // Get the audio data that corresponds to the current section
            if let Some(data) = audio_data.get_sample(data_start..data_end) {
                if data.is_empty() {
                    current_write_index += rendered_frames * MAX_CHANNELS;
                    continue;
                };

                // Calculate the resample ratio based on the sample rate ratio
                // Include BPM in the calculate only in the Musical region bounds
                let resample_ratio = match self.bounds {
                    TimeBounds::Musical { .. } => sample_rate_ratio * self.bpm / section.bpm,
                    TimeBounds::Time { .. } => sample_rate_ratio,
                };

                // Resample the audio data based on the resample ratio calculated by the tempo map
                if (resample_ratio - 1.0).abs() < 1e-6 {
                    // Interleave and add the resampled data to the buffer, which must have MAX_CHANNELS channels
                    if current_write_index < buffer.len() {
                        add_samples_interleaved(
                            data,
                            &mut buffer[current_write_index..],
                            audio_data.info.channels,
                            MAX_CHANNELS,
                        );
                    }
                } else {
                    resample_channels(
                        data,
                        &mut self.resampled_buffer,
                        rendered_frames,
                        audio_data.info.channels,
                        resample_ratio,
                    );

                    // Interleave and add the resampled data to the buffer, which must have MAX_CHANNELS channels
                    if current_write_index < buffer.len() {
                        add_samples_interleaved(
                            &self.resampled_buffer,
                            &mut buffer[current_write_index..],
                            audio_data.info.channels,
                            MAX_CHANNELS,
                        );
                    }
                };
            }

            // Advance the destination offset
            current_write_index += rendered_frames * MAX_CHANNELS;
        }
    }
}

/// Add the samples from the source buffer to the destination buffer while interleaving the channels
/// with the given number of source and destination channels.
pub(super) fn add_samples_interleaved(
    source: &[f32],
    destination: &mut [f32],
    src_channels: usize,
    dst_channels: usize,
) {
    let active_channels = src_channels.min(dst_channels);

    // Finally add the output data to the output buffer while interleaving the channels
    for (dst_frame, src_frame) in destination
        .chunks_exact_mut(dst_channels)
        .zip(source.chunks_exact(src_channels))
    {
        for ch in 0..active_channels {
            // Add the sample value
            dst_frame[ch] += src_frame[ch];
        }
    }
}
