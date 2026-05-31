use crate::load_write::{AsBytes, FromBytes};
use kreniq_engine::data_types::AudioContext;
use std::io::{Cursor, Read};

impl AsBytes for AudioContext {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write the audio configurations
        bytes.extend(&(self.channels as u64).to_le_bytes());
        bytes.extend(&(self.sample_rate as u64).to_le_bytes());
        bytes.extend(&(self.buffer_size as u64).to_le_bytes());
        bytes.extend(&(self.max_voices as u64).to_le_bytes());
    }
}

impl FromBytes for AudioContext {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(bytes);

        // Read the audio configurations from the bytes
        let mut channels_bytes = [0u8; 8];
        let mut sample_rate_bytes = [0u8; 8];
        let mut buffer_size_bytes = [0u8; 8];
        let mut max_voices_bytes = [0u8; 8];
        cursor.read_exact(&mut channels_bytes)?;
        cursor.read_exact(&mut sample_rate_bytes)?;
        cursor.read_exact(&mut buffer_size_bytes)?;
        cursor.read_exact(&mut max_voices_bytes)?;
        let channels = u64::from_le_bytes(channels_bytes) as usize;
        let sample_rate = u64::from_le_bytes(sample_rate_bytes) as usize;
        let buffer_size = u64::from_le_bytes(buffer_size_bytes) as usize;
        let max_voices = u64::from_le_bytes(max_voices_bytes) as usize;

        // Construct the new audio context
        let audio_ctx = AudioContext {
            channels,
            sample_rate,
            buffer_size,
            max_voices,
        };
        Ok(audio_ctx)
    }
}
