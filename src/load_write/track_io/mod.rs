mod audio_track;
mod note_track;

use crate::load_write::{AsBytes, FromBytes, safe_read};
use kreniq_engine::{
    graph::Graph,
    track::{Track, audio_track::AudioTrack, note_track::NoteTrack},
};
use std::io::{Cursor, Read};

#[repr(u8)]
enum TrackKind {
    Audio = 0,
    Note = 1,
}

impl AsBytes for dyn Track {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write the graph and its length
        let mut graph_bytes = Vec::new();
        self.get_graph().as_bytes(&mut graph_bytes);
        bytes.extend((graph_bytes.len() as u64).to_le_bytes());
        bytes.extend(graph_bytes);

        // Then write the contents of the track depending on the track type
        if let Some(audio_track) = self.as_any().downcast_ref::<AudioTrack>() {
            // Write the track kind
            bytes.push(TrackKind::Audio as u8);

            // Get the size of the track
            let mut track_bytes = Vec::new();
            audio_track.as_bytes(&mut track_bytes);
            // Write the size of the track and its contents
            bytes.extend((track_bytes.len() as u64).to_le_bytes());
            bytes.extend(track_bytes);
        } else if let Some(note_track) = self.as_any().downcast_ref::<NoteTrack>() {
            // Write the track kind
            bytes.push(TrackKind::Note as u8);

            // Get the size of the track
            let mut track_bytes = Vec::new();
            note_track.as_bytes(&mut track_bytes);
            // Write the size of the track and its contents
            bytes.extend((track_bytes.len() as u64).to_le_bytes());
            bytes.extend(track_bytes);
        }
    }
}

impl FromBytes for Box<dyn Track> {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(bytes);

        // Read the length of the graph data
        let mut graph_len_bytes = [0u8; 8];
        cursor.read_exact(&mut graph_len_bytes)?;
        let graph_len = u64::from_le_bytes(graph_len_bytes) as usize;

        // Read the graph data
        let graph_data_bytes = safe_read(&mut cursor, graph_len)?;
        let graph = Graph::from_bytes(&graph_data_bytes)?;

        // Get the first one byte and get the type of the track
        let mut type_byte = [0u8; 1];
        let mut track_len_bytes = [0u8; 8];
        cursor.read_exact(&mut type_byte)?;
        cursor.read_exact(&mut track_len_bytes)?;
        let track_len = u64::from_le_bytes(track_len_bytes) as usize;

        // Get the content bytes
        let track_data_bytes = safe_read(&mut cursor, track_len)?;

        let mut track: Box<dyn Track> = match type_byte[0] {
            0 => Box::new(AudioTrack::from_bytes(&track_data_bytes)?),
            1 => Box::new(NoteTrack::from_bytes(&track_data_bytes)?),
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid track kind",
                ));
            }
        };
        track.set_graph(graph);

        Ok(track)
    }
}
