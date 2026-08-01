use crate::storage::project::{
    AsBytes, FromBytes,
    error::{Contextualize, LoadError, ParseContext},
};
use kadent_engine::{data_types::Ticks, track::audio_track::AudioRegion};
use std::io::Read;

impl AsBytes for AudioRegion {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write the audio data
        let data_bytes = self
            .data
            .iter()
            .flat_map(|sample| sample.to_le_bytes())
            .collect::<Vec<u8>>();

        // Write the metadata
        bytes.extend(self.sample_rate.to_le_bytes());
        bytes.extend(self.channels.to_le_bytes());
        bytes.extend(self.base_bpm.to_le_bytes());
        bytes.extend(self.start.0.to_le_bytes());
        bytes.extend(self.duration.0.to_le_bytes());
        bytes.extend(self.max_duration.0.to_le_bytes());
        // Write the length of the audio data
        bytes.extend((data_bytes.len() as u64).to_le_bytes());
        // Then write the data
        bytes.extend_from_slice(&data_bytes);
    }
}

impl FromBytes for AudioRegion {
    fn from_bytes(bytes: &[u8]) -> Result<Self, LoadError> {
        let mut cursor = std::io::Cursor::new(bytes);

        // Read the metadata
        let mut sample_rate_bytes = [0u8; 4];
        let mut channels_bytes = [0u8; 2];
        let mut base_bpm_bytes = [0u8; 8];
        let mut start_bytes = [0u8; 8];
        let mut duration_bytes = [0u8; 8];
        let mut max_duration_bytes = [0u8; 8];
        let mut data_len_bytes = [0u8; 8];

        cursor
            .read_exact(&mut sample_rate_bytes)
            .with_ctx(ParseContext::AudioRegion)?;
        cursor
            .read_exact(&mut channels_bytes)
            .with_ctx(ParseContext::AudioRegion)?;
        cursor
            .read_exact(&mut base_bpm_bytes)
            .with_ctx(ParseContext::AudioRegion)?;
        cursor
            .read_exact(&mut start_bytes)
            .with_ctx(ParseContext::AudioRegion)?;
        cursor
            .read_exact(&mut duration_bytes)
            .with_ctx(ParseContext::AudioRegion)?;
        cursor
            .read_exact(&mut max_duration_bytes)
            .with_ctx(ParseContext::AudioRegion)?;
        cursor
            .read_exact(&mut data_len_bytes)
            .with_ctx(ParseContext::AudioRegion)?;

        let sample_rate = u32::from_le_bytes(sample_rate_bytes);
        let channels = u16::from_le_bytes(channels_bytes);
        let base_bpm = f64::from_le_bytes(base_bpm_bytes);
        let start = Ticks(i64::from_le_bytes(start_bytes));
        let duration = Ticks(i64::from_le_bytes(duration_bytes));
        let max_duration = Ticks(i64::from_le_bytes(max_duration_bytes));
        let data_len = u64::from_le_bytes(data_len_bytes) as usize;

        // Read the audio data
        let mut data_bytes = vec![0u8; data_len];
        cursor
            .read_exact(&mut data_bytes)
            .with_ctx(ParseContext::AudioRegion)?;

        // Convert bytes to f32 samples
        let data: Vec<f32> = data_bytes
            .chunks_exact(4)
            .map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap()))
            .collect();
        let frames = data.len() / channels as usize;

        Ok(AudioRegion {
            data,
            frames,
            sample_rate,
            channels,
            base_bpm,
            start,
            duration,
            max_duration,
        })
    }
}
