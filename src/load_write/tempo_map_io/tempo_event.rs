use crate::load_write::{AsBytes, FromBytes};
use kreniq_engine::{data_types::Beats, mixer::TempoEvent};
use std::io::{Cursor, Read};

impl AsBytes for TempoEvent {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        bytes.extend(&self.beat.0.to_le_bytes());
        bytes.extend(&self.bpm.to_le_bytes());
        bytes.extend(&(self.sample_offset as u64).to_le_bytes());
    }
}

impl FromBytes for TempoEvent {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(bytes);

        let mut beat_bytes = [0u8; 8];
        let mut bpm_bytes = [0u8; 8];
        let mut sample_offset_bytes = [0u8; 8];
        cursor.read_exact(&mut beat_bytes)?;
        cursor.read_exact(&mut bpm_bytes)?;
        cursor.read_exact(&mut sample_offset_bytes)?;
        let beat = Beats(f64::from_le_bytes(beat_bytes));
        let bpm = f64::from_le_bytes(bpm_bytes);
        let sample_offset = u64::from_le_bytes(sample_offset_bytes) as usize;
        Ok(TempoEvent {
            beat,
            bpm,
            sample_offset,
        })
    }
}
