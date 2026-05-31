mod note;
mod note_region;

use crate::load_write::{AsBytes, FromBytes, safe_read};
use kreniq_engine::track::{
    RegionID,
    note_track::{NoteRegion, NoteTrack},
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

        // Read the track bytes
        let regions_data_bytes = safe_read(&mut cursor, region_len)?;

        // Parse the regions
        let mut regions = HashMap::new();
        let mut region_cursor = Cursor::new(regions_data_bytes.as_slice());
        while region_cursor.position() < region_len as u64 {
            // Get the length of the region and the region ID
            let mut region_len_bytes = [0u8; 8];
            let mut region_id_bytes = [0u8; 8];
            region_cursor.read_exact(&mut region_len_bytes)?;
            region_cursor.read_exact(&mut region_id_bytes)?;
            let region_len = u64::from_le_bytes(region_len_bytes) as usize;
            let region_id = RegionID(u64::from_le_bytes(region_id_bytes) as usize);

            // Get the region content data and decode it
            let region_data_bytes = safe_read(&mut region_cursor, region_len)?;
            let region = NoteRegion::from_bytes(&region_data_bytes)?;

            regions.insert(region_id, region);
        }

        // Construct a new track
        let mut track = NoteTrack::default();
        track.set_regions(regions);
        restore_next_region_id(&mut track);

        Ok(track)
    }
}

fn restore_next_region_id(track: &mut NoteTrack) {
    let next_id = track
        .get_all_regions()
        .keys()
        .map(|id| id.0)
        .max()
        .map(|m| m + 1)
        .unwrap_or(0);
    track.set_next_region_id(next_id);
}
