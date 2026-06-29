use crate::storage::project::{
    AsBytes, FromBytes,
    error::{Contextualize, LoadError, ParseContext},
    safe_read,
};
use kadent_engine::{
    data_types::Ticks,
    track::note_track::{Note, NoteID, NoteRegion},
};
use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

impl AsBytes for NoteRegion {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write the start and duration
        bytes.extend(self.start.0.to_le_bytes());
        bytes.extend(self.duration.0.to_le_bytes());

        // Convert the notes into bytes
        let mut note_bytes = Vec::new();
        for (note_id, note) in &self.notes {
            note_bytes.extend(note_id.0.to_le_bytes());
            note.as_bytes(&mut note_bytes);
        }

        // Write the length of the note bytes
        bytes.extend((note_bytes.len() as u64).to_le_bytes());
        // Write the note bytes
        bytes.extend(note_bytes);
    }
}

impl FromBytes for NoteRegion {
    fn from_bytes(bytes: &[u8]) -> Result<Self, LoadError> {
        let mut cursor = Cursor::new(bytes);

        // Read the start and duration
        let mut start_bytes = [0u8; 8];
        let mut duration_bytes = [0u8; 8];
        cursor
            .read_exact(&mut start_bytes)
            .with_ctx(ParseContext::NoteRegion)?;
        cursor
            .read_exact(&mut duration_bytes)
            .with_ctx(ParseContext::NoteRegion)?;
        let start = Ticks(i64::from_le_bytes(start_bytes));
        let duration = Ticks(i64::from_le_bytes(duration_bytes));

        // Read the length of the note bytes
        let mut note_bytes = [0u8; 8];
        cursor
            .read_exact(&mut note_bytes)
            .with_ctx(ParseContext::NoteRegion)?;
        let notes_len = u64::from_le_bytes(note_bytes) as usize;

        // Read the note bytes
        let notes_data_bytes =
            safe_read(&mut cursor, notes_len).with_ctx(ParseContext::NoteRegion)?;

        // Parse the notes
        let mut notes = HashMap::new();
        let mut note_cursor = Cursor::new(&notes_data_bytes);
        while note_cursor.position() < notes_len as u64 {
            let mut note_id_bytes = [0u8; 8];
            let mut note_data_bytes = [0u8; 24];
            note_cursor
                .read_exact(&mut note_id_bytes)
                .with_ctx(ParseContext::NoteRegion)?;
            note_cursor
                .read_exact(&mut note_data_bytes)
                .with_ctx(ParseContext::NoteRegion)?;

            let note_id = u64::from_le_bytes(note_id_bytes);
            let note = Note::from_bytes(&note_data_bytes)?;

            notes.insert(NoteID(note_id), note);
        }

        // Construct a new note region
        let mut region = NoteRegion::new(start, duration);
        region.notes = notes;
        restore_next_note_id(&mut region);

        Ok(region)
    }
}

fn restore_next_note_id(region: &mut NoteRegion) {
    let next_id = region
        .notes
        .keys()
        .map(|id| id.0)
        .max()
        .map(|m| m + 1)
        .unwrap_or(0);
    region.set_next_note_id(next_id);
}
