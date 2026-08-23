use crate::core::audio_engine::MAX_CHANNELS;

/// A struct that represents a single sample in a audio buffer.
/// This struct can be transferred to and from the KASL language as a `Sample` type.
#[repr(C)]
#[derive(Clone, Debug)]
pub(crate) struct Sample([f32; MAX_CHANNELS]);

impl Default for Sample {
    fn default() -> Self {
        Self([0.0; MAX_CHANNELS])
    }
}
