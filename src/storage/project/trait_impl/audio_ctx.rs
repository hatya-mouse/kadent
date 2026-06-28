use crate::storage::project::{
    AsBytes, FromBytes,
    error::{Contextualize, LoadError, ParseContext},
};
use kadent_engine::data_types::AudioContext;
use std::io::{Cursor, Read};

impl AsBytes for AudioContext {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write the audio configurations
        bytes.extend(&(self.resolution).to_le_bytes());
        bytes.extend(&(self.channels as u64).to_le_bytes());
        bytes.extend(&(self.sample_rate).to_le_bytes());
        bytes.extend(&(self.buffer_size as u64).to_le_bytes());
        bytes.extend(&(self.max_voices as u64).to_le_bytes());
    }
}

impl FromBytes for AudioContext {
    fn from_bytes(bytes: &[u8]) -> Result<Self, LoadError> {
        let mut cursor = Cursor::new(bytes);

        // Read the audio configurations from the bytes
        let mut resolution_bytes = [0u8; 8];
        let mut channels_bytes = [0u8; 8];
        let mut sample_rate_bytes = [0u8; 8];
        let mut buffer_size_bytes = [0u8; 8];
        let mut max_voices_bytes = [0u8; 8];
        cursor
            .read_exact(&mut resolution_bytes)
            .with_ctx(ParseContext::AudioContext)?;
        cursor
            .read_exact(&mut channels_bytes)
            .with_ctx(ParseContext::AudioContext)?;
        cursor
            .read_exact(&mut sample_rate_bytes)
            .with_ctx(ParseContext::AudioContext)?;
        cursor
            .read_exact(&mut buffer_size_bytes)
            .with_ctx(ParseContext::AudioContext)?;
        cursor
            .read_exact(&mut max_voices_bytes)
            .with_ctx(ParseContext::AudioContext)?;
        let resolution = u64::from_le_bytes(channels_bytes);
        let channels = u64::from_le_bytes(channels_bytes) as usize;
        let sample_rate = u64::from_le_bytes(sample_rate_bytes);
        let buffer_size = u64::from_le_bytes(buffer_size_bytes) as usize;
        let max_voices = u64::from_le_bytes(max_voices_bytes) as usize;

        // Construct the new audio context
        let audio_ctx = AudioContext {
            resolution,
            channels,
            sample_rate,
            buffer_size,
            max_voices,
        };
        Ok(audio_ctx)
    }
}
