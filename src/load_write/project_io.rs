use crate::load_write::{AsBytes, FromBytes, safe_read};
use kreniq_engine::{
    data_types::{AudioContext, Beats},
    mixer::{Project, TempoMap, TrackID},
    track::Track,
};
use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

impl AsBytes for Project {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write the audio configurations
        self.audio_ctx.as_bytes(bytes);

        // Write the range of the song
        bytes.extend(&self.range_start.0.to_le_bytes());
        bytes.extend(&self.range_duration.0.to_le_bytes());

        // Write the tempo map and its size
        let mut tempo_map_bytes = Vec::new();
        self.tempo_map.as_bytes(&mut tempo_map_bytes);
        bytes.extend(&(tempo_map_bytes.len() as u64).to_le_bytes());
        bytes.extend(tempo_map_bytes);

        // Write the tracks to the temporary buffer
        let mut tracks_bytes = Vec::new();
        for (track_id, track) in &self.tracks {
            // Get the size of the track data
            let mut track_bytes = Vec::new();
            track.as_bytes(&mut track_bytes);

            // Write the track ID, the size of the track and its contents
            tracks_bytes.extend(&(track_id.0 as u64).to_le_bytes());
            tracks_bytes.extend(&(track_bytes.len() as u64).to_le_bytes());
            tracks_bytes.extend(track_bytes);
        }

        // Write the size of all tracks and the tracks themselves
        bytes.extend(&(tracks_bytes.len() as u64).to_le_bytes());
        bytes.extend(tracks_bytes);
    }
}

impl FromBytes for Project {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(bytes);

        // Read the audio configurations from the bytes
        let mut audio_ctx_bytes = [0u8; 32];
        cursor.read_exact(&mut audio_ctx_bytes)?;
        let audio_ctx = AudioContext::from_bytes(&audio_ctx_bytes)?;

        // Read the range of the song
        let mut range_start_bytes = [0u8; 8];
        let mut range_duration_bytes = [0u8; 8];
        cursor.read_exact(&mut range_start_bytes)?;
        cursor.read_exact(&mut range_duration_bytes)?;
        let range_start = Beats(f64::from_le_bytes(range_start_bytes));
        let range_duration = Beats(f64::from_le_bytes(range_duration_bytes));

        // Read the tempo map from the bytes
        let mut tempo_map_len_bytes = [0u8; 8];
        cursor.read_exact(&mut tempo_map_len_bytes)?;
        let tempo_map_len = u64::from_le_bytes(tempo_map_len_bytes) as usize;
        let tempo_map_bytes = safe_read(&mut cursor, tempo_map_len)?;
        let mut tempo_map = TempoMap::from_bytes(&tempo_map_bytes)?;
        tempo_map.set_audio_ctx(audio_ctx.clone());

        // Read the length of all tracks
        let mut tracks_len_bytes = [0u8; 8];
        cursor.read_exact(&mut tracks_len_bytes)?;
        let tracks_len = u64::from_le_bytes(tracks_len_bytes);

        // Read the tracks data
        let tracks_bytes = safe_read(&mut cursor, tracks_len as usize)?;

        let mut tracks = HashMap::new();
        let mut tracks_cursor = Cursor::new(tracks_bytes.as_slice());
        while tracks_cursor.position() < tracks_len {
            // Get the ID and the length of the track contents
            let mut track_id_bytes = [0u8; 8];
            let mut data_len_bytes = [0u8; 8];
            tracks_cursor.read_exact(&mut track_id_bytes)?;
            tracks_cursor.read_exact(&mut data_len_bytes)?;
            let track_id = TrackID(u64::from_le_bytes(track_id_bytes) as usize);
            let data_len = u64::from_le_bytes(data_len_bytes) as usize;

            // Parse the track contents
            let track_data_bytes = safe_read(&mut tracks_cursor, data_len)?;
            let track = <Box<dyn Track>>::from_bytes(&track_data_bytes)?;

            tracks.insert(track_id, track);
        }

        // Construct the new project
        let mut project =
            Project::with_tempo_map(audio_ctx.clone(), tempo_map, range_start, range_duration);
        project.tracks = tracks;
        restore_next_track_id(&mut project);

        // Restore the audio context on deserialized tracks
        for track in project.tracks.values_mut() {
            track.set_audio_ctx(&audio_ctx);
        }

        Ok(project)
    }
}

fn restore_next_track_id(project: &mut Project) {
    let next_id = project
        .tracks
        .keys()
        .map(|id| id.0)
        .max()
        .map(|m| m + 1)
        .unwrap_or(0);
    project.set_next_track_id(next_id);
}
