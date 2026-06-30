use crate::storage::project::{
    AsBytes, FromBytes,
    error::{Contextualize, LoadError, ParseContext},
};
use kadent_engine::data_types::ProjectConfig;
use std::io::{Cursor, Read};

impl AsBytes for ProjectConfig {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write the audio configurations
        bytes.extend(&(self.resolution).to_le_bytes());
        bytes.extend(&(self.channels).to_le_bytes());
    }
}

impl FromBytes for ProjectConfig {
    fn from_bytes(bytes: &[u8]) -> Result<Self, LoadError> {
        let mut cursor = Cursor::new(bytes);

        // Read the audio configurations from the bytes
        let mut resolution_bytes = [0u8; 8];
        let mut channels_bytes = [0u8; 2];
        cursor
            .read_exact(&mut resolution_bytes)
            .with_ctx(ParseContext::ProjectConfig)?;
        cursor
            .read_exact(&mut channels_bytes)
            .with_ctx(ParseContext::ProjectConfig)?;
        let resolution = u64::from_le_bytes(resolution_bytes);
        let channels = u16::from_le_bytes(channels_bytes);

        // Construct the new audio context
        let audio_ctx = ProjectConfig {
            resolution,
            channels,
        };
        Ok(audio_ctx)
    }
}
