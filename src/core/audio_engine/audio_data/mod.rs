mod audio_pool;
mod source;

use std::slice::SliceIndex;

pub(crate) use audio_pool::AudioFilePool;
pub(crate) use source::AudioSource;

#[derive(Debug, Clone)]
pub(crate) struct AudioDataInfo {
    /// The number of channels in the audio data.
    pub(crate) channels: usize,
    /// The number of samples in the audio data.
    pub(crate) frames: usize,
    /// The sample rate associated with the audio data.
    pub(crate) sample_rate: u64,
}

/// The raw audio data stored as a vector of f32 samples.
#[derive(Debug, Clone)]
pub(crate) struct AudioData {
    /// The raw audio samples.
    pub(crate) samples: Vec<f32>,
    /// The information about the audio data.
    pub(crate) info: AudioDataInfo,
}

impl Default for AudioData {
    fn default() -> Self {
        Self {
            samples: Vec::new(),
            info: AudioDataInfo {
                channels: 0,
                frames: 0,
                sample_rate: 0,
            },
        }
    }
}

impl AudioData {
    pub(crate) fn new(samples: Vec<f32>, info: AudioDataInfo) -> Self {
        Self { samples, info }
    }

    /// Get the slice of interleaves samples in the requested range.
    pub(crate) fn get_sample<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[f32]>,
    {
        self.samples.get(index)
    }

    /// Returns the slice of the entire samples.
    pub(crate) fn all_samples(&self) -> &[f32] {
        &self.samples
    }
}
