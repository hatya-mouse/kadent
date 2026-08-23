/// Resamples the audio data from the source buffer using linear interpolation and writes the result to the destination buffer.
///
/// # Parameters
/// - `source`: A slice of f32 values representing the source audio data (interleaved).
/// - `destination_buffer`: A mutable reference to a vector of f32 values where the result will be stored.
/// - `dst_samples`: The number of samples to write to the destination buffer.
/// - `channels`: The number of channels in the audio data.
/// - `ratio`: The resampling ratio (destination sample rate / source sample rate).
pub(crate) fn resample_channels(
    source: &[f32],
    destination_buffer: &mut Vec<f32>,
    dst_samples: usize,
    channels: usize,
    ratio: f64,
) {
    if source.is_empty() || channels == 0 || ratio == 0.0 {
        return;
    }
    let src_samples = source.len() / channels;

    // Calculate the length of the output array (interleaved) and fill it with zeros
    let full_len = dst_samples * channels;
    destination_buffer.resize(full_len, 0f32);

    // Loop through each sample in the target array
    let mut src_pos = 0.0;
    destination_buffer
        .chunks_mut(channels)
        .for_each(|sample_buffer| {
            // Calculate the corresponding position in the source array
            let index = src_pos as usize;

            if index + 1 < src_samples {
                let gradient = (src_pos - index as f64) as f32;

                // Get indices to interpolate between
                let src_index_0 = index * channels;
                let src_index_1 = src_index_0 + channels;

                for channel in 0..channels {
                    let src_0 = source[src_index_0 + channel];
                    let src_1 = source[src_index_1 + channel];
                    sample_buffer[channel] = src_0 + (src_1 - src_0) * gradient;
                }
            } else {
                // If the index is out of bounds, write the last sample
                let last_index = (src_samples - 1) * channels;
                sample_buffer[..channels]
                    .clone_from_slice(&source[last_index..last_index + channels]);
            }

            // Advance the source position by the resampling ratio
            src_pos += ratio;
        });
}
