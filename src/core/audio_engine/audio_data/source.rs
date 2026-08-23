use hound::SampleFormat;
use serde::{Deserialize, Serialize};
use std::{
    ops::Range,
    path::{Path, PathBuf},
};

use crate::core::audio_engine::audio_data::{AudioData, AudioDataInfo};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) enum AudioSource {
    /// The audio data is loaded from the original audio file.
    Original(PathBuf),
    /// The audio data is loaded from a modified copy of the file.
    Modified(PathBuf),
    /// The audio data is empty.
    Zero,
}

impl AudioSource {
    /// Returns the full interleaved audio data.
    pub(crate) fn get_data(&self) -> Option<AudioData> {
        self.get_data_in(0..usize::MAX)
    }

    /// Returns the interleaved audio data in the range.
    pub(crate) fn get_data_in(&self, range: Range<usize>) -> Option<AudioData> {
        match self {
            AudioSource::Original(path) => load_from_path(path, range),
            AudioSource::Modified(path) => load_from_path(path, range),
            AudioSource::Zero => None,
        }
    }
}

fn load_from_path(path: &Path, range: Range<usize>) -> Option<AudioData> {
    let mut reader = hound::WavReader::open(path).ok()?;
    let spec = reader.spec();

    let channels = spec.channels as usize;
    let sample_count = range
        .end
        .saturating_sub(range.start)
        .saturating_mul(channels);

    // Seek to the first sample in the range
    reader.seek(range.start as u32).ok()?;

    // Create a audio data information
    let info = AudioDataInfo {
        channels,
        frames: sample_count,
        sample_rate: spec.sample_rate as u64,
    };

    // Then read the samples in the range
    let data = match spec.sample_format {
        SampleFormat::Float => reader
            .into_samples::<f32>()
            .take(sample_count)
            .filter_map(Result::ok)
            .collect(),
        SampleFormat::Int => {
            let max_value = 2_i32.pow(spec.bits_per_sample as u32 - 1) as f32;
            let inv_max_value = 1.0 / max_value;
            reader
                .into_samples::<i32>()
                .take(sample_count)
                .filter_map(Result::ok)
                .map(|s| s as f32 * inv_max_value)
                .collect()
        }
    };

    Some(AudioData::new(data, info))
}
