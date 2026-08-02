use crate::background_thread::commands::{WaveformLod, WaveformPeaks};

const SMALL_BLOCK_SIZE: usize = 64;
const MEDIUM_BLOCK_SIZE: usize = 512;
const LARGE_BLOCK_SIZE: usize = 4096;

/// Computes min/max peaks at three fixed resolutions from interleaved multi-channel samples.
pub(super) fn run_generate_waveform(samples: &[f32], channels: u16) -> WaveformLod {
    WaveformLod {
        small: compute_peaks(samples, channels, SMALL_BLOCK_SIZE),
        medium: compute_peaks(samples, channels, MEDIUM_BLOCK_SIZE),
        large: compute_peaks(samples, channels, LARGE_BLOCK_SIZE),
    }
}

/// Computes min/max peak values for a given block size from interleaved multi-channel samples.
fn compute_peaks(samples: &[f32], channels: u16, block_size: usize) -> WaveformPeaks {
    let channels = channels.max(1) as usize;
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
