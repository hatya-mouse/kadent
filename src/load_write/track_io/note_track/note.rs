use crate::load_write::{AsBytes, FromBytes};
use kreniq_engine::{data_types::Beats, track::note_track::Note};
use std::io::{Cursor, Read};

impl AsBytes for Note {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        bytes.extend(self.start.0.to_le_bytes());
        bytes.extend(self.duration.0.to_le_bytes());
        bytes.extend(self.pitch.to_le_bytes());
        bytes.extend(self.velocity.to_le_bytes());
    }
}

impl FromBytes for Note {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(bytes);

        // Get the start beats, duration, pitch and velocity from the bytes
        let mut start_bytes = [0u8; 8];
        let mut duration_bytes = [0u8; 8];
        let mut pitch_bytes = [0u8; 4];
        let mut velocity_bytes = [0u8; 4];

        cursor.read_exact(&mut start_bytes)?;
        cursor.read_exact(&mut duration_bytes)?;
        cursor.read_exact(&mut pitch_bytes)?;
        cursor.read_exact(&mut velocity_bytes)?;

        let start = Beats(f64::from_le_bytes(start_bytes));
        let duration = Beats(f64::from_le_bytes(duration_bytes));
        let pitch = f32::from_le_bytes(pitch_bytes);
        let velocity = f32::from_le_bytes(velocity_bytes);

        Ok(Note::new(start, duration, pitch, velocity))
    }
}
