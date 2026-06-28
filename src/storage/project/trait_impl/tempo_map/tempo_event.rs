use crate::storage::project::{
    AsBytes, FromBytes,
    error::{Contextualize, LoadError, ParseContext},
};
use kadent_engine::{data_types::Ticks, mixer::TempoEvent};
use std::io::{Cursor, Read};

impl AsBytes for TempoEvent {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        bytes.extend(&self.ticks().0.to_le_bytes());
        bytes.extend(&self.bpm().to_le_bytes());
        bytes.extend(&(self.sample_offset() as u64).to_le_bytes());
    }
}

impl FromBytes for TempoEvent {
    fn from_bytes(bytes: &[u8]) -> Result<Self, LoadError> {
        let mut cursor = Cursor::new(bytes);

        let mut ticks_bytes = [0u8; 8];
        let mut bpm_bytes = [0u8; 8];
        let mut sample_offset_bytes = [0u8; 8];
        cursor
            .read_exact(&mut ticks_bytes)
            .with_ctx(ParseContext::TempoEvent)?;
        cursor
            .read_exact(&mut bpm_bytes)
            .with_ctx(ParseContext::TempoEvent)?;
        cursor
            .read_exact(&mut sample_offset_bytes)
            .with_ctx(ParseContext::TempoEvent)?;
        let ticks = Ticks(u64::from_le_bytes(ticks_bytes));
        let bpm = f64::from_le_bytes(bpm_bytes);
        let sample_offset = u64::from_le_bytes(sample_offset_bytes) as usize;
        Ok(TempoEvent::new(ticks, bpm, sample_offset))
    }
}
