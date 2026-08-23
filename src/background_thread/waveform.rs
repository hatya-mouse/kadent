use crate::core::audio_engine::audio_data::AudioSource;
use crate::{
    background_thread::commands::{WaveformLod, WaveformPeaks},
    consts::{LARGE_BLOCK_SIZE, MEDIUM_BLOCK_SIZE, SMALL_BLOCK_SIZE},
};

/// Computes min/max peaks at three fixed resolutions from interleaved multi-channel samples.
pub(super) fn run_generate_waveform(source: &AudioSource) -> WaveformLod {
    let data = source.get_data().unwrap_or_default();
    let channels = data.info.channels;
    let samples = data.all_samples();

    WaveformLod {
        data: data.clone(),
        small: compute_peaks(samples, channels, SMALL_BLOCK_SIZE),
        medium: compute_peaks(samples, channels, MEDIUM_BLOCK_SIZE),
        large: compute_peaks(samples, channels, LARGE_BLOCK_SIZE),
    }
}

/// Computes min/max peak values for a given block size from interleaved multi-channel samples.
fn compute_peaks(samples: &[f32], channels: usize, block_size: usize) -> WaveformPeaks {
    let channels = channels.max(1);
    let frame_count = samples.len() / channels;

    let peaks = (0..frame_count)
        .step_by(block_size)
        .map(|start_frame| {
            let end_frame = (start_frame + block_size).min(frame_count);
            let start = start_frame * channels;
            let end = end_frame * channels;

            samples[start..end]
                .iter()
                .fold((0.0f32, 0.0f32), |(min, max), &s| (min.min(s), max.max(s)))
        })
        .collect();

    WaveformPeaks { peaks }
}
