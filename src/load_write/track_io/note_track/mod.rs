mod note;
mod note_region;

use crate::load_write::{AsBytes, FromBytes};
use knodiq_engine::{
    data_types::AudioContext,
    track::{
        RegionID,
        note_track::{NoteRegion, NoteTrack},
    },
};
use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

impl AsBytes for NoteTrack {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Convert the regions into bytes
        let mut regions_bytes = Vec::new();
        for (region_id, region) in self.get_all_regions() {
            let mut region_bytes = Vec::new();
            region.as_bytes(&mut region_bytes);

            // Store the length of the region and the region itself
            regions_bytes.extend((region_bytes.len() as u64).to_le_bytes());
            regions_bytes.extend((region_id.0 as u64).to_le_bytes());
            regions_bytes.extend(region_bytes);
        }

        // Write the length of the regions
        bytes.extend((regions_bytes.len() as u64).to_le_bytes());
        // Write the regions
        bytes.extend(regions_bytes);
    }
}

impl FromBytes for NoteTrack {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(bytes);

        // Read the length of the regions
        let mut region_len_bytes = [0u8; 8];
        cursor.read_exact(&mut region_len_bytes)?;
        let region_len = u64::from_le_bytes(region_len_bytes) as usize;

        // Read the note bytes
        let mut regions_data_bytes = vec![0u8; region_len];
        cursor.read_exact(&mut regions_data_bytes)?;

        // Parse the regions
        let mut regions = HashMap::new();
        let mut region_cursor = Cursor::new(bytes);
        while region_cursor.position() < region_len as u64 {
            // Get the length of the region and the region ID
            let mut region_len_bytes = [0u8; 8];
            let mut region_id_bytes = [0u8; 8];
            region_cursor.read_exact(&mut region_len_bytes)?;
            region_cursor.read_exact(&mut region_id_bytes)?;
            let region_len = u64::from_le_bytes(region_len_bytes) as usize;
            let region_id = RegionID(u64::from_le_bytes(region_id_bytes) as usize);

            // Get the region content data and decode it
            let mut region_data_bytes = vec![0u8; region_len];
            region_cursor.read_exact(&mut region_data_bytes)?;
            let region = NoteRegion::from_bytes(&region_data_bytes)?;

            regions.insert(region_id, region);
        }

        // Construct a new track
        let mut track = NoteTrack::new(AudioContext::default());
        track.set_regions(regions);

        Ok(track)
    }
}
