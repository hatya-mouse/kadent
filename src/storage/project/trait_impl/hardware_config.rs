use crate::storage::project::{
    AsBytes, FromBytes,
    error::{Contextualize, LoadError, ParseContext},
};
use kadent_engine::data_types::HardwareConfig;
use std::io::{Cursor, Read};

impl AsBytes for HardwareConfig {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write the hardware configurations
        bytes.extend(&self.sample_rate.to_le_bytes());
        bytes.extend(&self.buffer_size.to_le_bytes());
        bytes.extend(&self.max_voices.to_le_bytes());
    }
}

impl FromBytes for HardwareConfig {
    fn from_bytes(bytes: &[u8]) -> Result<Self, LoadError> {
        let mut cursor = Cursor::new(bytes);

        // Read the audio configurations from the bytes
        let mut sample_rate_bytes = [0u8; 8];
        let mut buffer_size_bytes = [0u8; 4];
        let mut max_voices_bytes = [0u8; 2];
        cursor
            .read_exact(&mut sample_rate_bytes)
            .with_ctx(ParseContext::HardwareConfig)?;
        cursor
            .read_exact(&mut buffer_size_bytes)
            .with_ctx(ParseContext::HardwareConfig)?;
        cursor
            .read_exact(&mut max_voices_bytes)
            .with_ctx(ParseContext::HardwareConfig)?;
        let sample_rate = u64::from_le_bytes(sample_rate_bytes);
        let buffer_size = u32::from_le_bytes(buffer_size_bytes);
        let max_voices = u16::from_le_bytes(max_voices_bytes);

        // Construct the new audio context
        let hardware_config = HardwareConfig {
            sample_rate,
            buffer_size,
            max_voices,
        };
        Ok(hardware_config)
    }
}
