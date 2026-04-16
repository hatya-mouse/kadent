mod audio_track;
mod note_track;

use crate::load_write::{AsBytes, FromBytes};
use knodiq_engine::track::{Track, audio_track::AudioTrack, note_track::NoteTrack};
use std::io::{Cursor, Read};

#[repr(u8)]
enum TrackKind {
    Audio = 0,
    Note = 1,
}

impl AsBytes for dyn Track {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write the graph
        let graph = self.get_graph();
        graph.as_bytes(bytes);

        // Then write the contents of the track depending on the track type
        if let Some(audio_track) = self.as_any().downcast_ref::<AudioTrack>() {
            // Write the track kind
            bytes.push(TrackKind::Audio as u8);
            // Write the audio track contents
            audio_track.as_bytes(bytes);
        } else if let Some(note_track) = self.as_any().downcast_ref::<NoteTrack>() {
            // Write the track kind
            bytes.push(TrackKind::Note as u8);
            // Write the note track contents
            note_track.as_bytes(bytes);
        }
    }
}

impl FromBytes for Box<dyn Track> {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(bytes);

        // Get the first one byte and get the type of the track
        let mut type_byte = [0u8; 1];
        cursor.read_exact(&mut type_byte)?;

        match type_byte[0] {
            0 => Ok(Box::new(AudioTrack::from_bytes(&mut cursor)?)),
            1 => Ok(Box::new(NoteTrack::from_bytes(&mut cursor)?)),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid track kind",
            )),
        }
    }
}
