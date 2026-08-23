/// Converts seconds to samples with the given sample rate.
#[inline]
pub(crate) fn seconds_to_samples(seconds: f64, sample_rate: u64) -> usize {
    (seconds * sample_rate as f64).round() as usize
}

/// Converts samples to seconds with the given sample rate.
#[inline]
pub(crate) fn samples_to_seconds(samples: usize, sample_rate: u64) -> f64 {
    samples as f64 / sample_rate as f64
}

/// Converts the number of samples based on a given ratio.
/// The ratio must be calculated with the following formula:
///
/// ```
/// ratio = output_sample_rate / input_sample_rate
/// ```
#[inline]
pub(crate) fn convert_rate_with_ratio(samples: usize, ratio: f64) -> usize {
    (samples as f64 * ratio).round() as usize
}

/// Calculates the number of samples per tick based on the given sample rate, BPM, and resolution.
#[inline]
pub(crate) fn samples_per_tick(sample_rate: u64, bpm: f64, resolution: u64) -> f64 {
    60.0 * sample_rate as f64 / (bpm * resolution as f64)
}
