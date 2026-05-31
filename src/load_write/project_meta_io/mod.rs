mod graph_meta;
mod node_meta;
mod region_meta;
mod track_meta;

pub use graph_meta::StoredGraphMeta;
pub use node_meta::StoredNodeMeta;
pub use region_meta::StoredRegionMeta;
pub use track_meta::StoredTrackMeta;

use crate::{
    load_write::{AsBytes, FromBytes, traits::safe_read},
    metadata::ProjectMeta,
};
use kreniq_engine::mixer::TrackID;
use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

pub struct StoredProjMeta {
    pub track_metas: HashMap<TrackID, StoredTrackMeta>,
    pub kasl_search_paths: Vec<String>,
}

impl StoredProjMeta {
    pub fn from_proj_meta(proj_meta: &ProjectMeta) -> Self {
        let track_metas = proj_meta
            .tracks
            .iter()
            .map(|(track_id, track_meta)| (*track_id, StoredTrackMeta::from_track_meta(track_meta)))
            .collect();

        Self {
            track_metas,
            kasl_search_paths: proj_meta.kasl_search_paths.clone(),
        }
    }
}

impl AsBytes for StoredProjMeta {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write the track metadatas
        let mut track_metas_bytes = Vec::new();
        for (track_id, track_meta) in &self.track_metas {
            // Get the length of the track metadata in bytes
            let mut track_meta_bytes = Vec::new();
            track_meta.as_bytes(&mut track_meta_bytes);

            // Write the track ID and the length of the track metadata
            track_metas_bytes.extend(&(track_id.0 as u64).to_le_bytes());
            track_metas_bytes.extend(&(track_meta_bytes.len() as u64).to_le_bytes());
            track_metas_bytes.extend(track_meta_bytes);
        }

        // Write the length of all track metadatas and the track metadatas
        bytes.extend(&(track_metas_bytes.len() as u64).to_le_bytes());
        bytes.extend(track_metas_bytes);

        // Write the kasl search paths
        let mut kasl_search_paths_bytes: Vec<u8> = Vec::new();
        for path in &self.kasl_search_paths {
            let path_bytes = path.as_bytes();
            kasl_search_paths_bytes.extend(&(path_bytes.len() as u64).to_le_bytes());
            kasl_search_paths_bytes.extend(path_bytes);
        }
        bytes.extend(&(kasl_search_paths_bytes.len() as u64).to_le_bytes());
        bytes.extend(kasl_search_paths_bytes);
    }
}

impl FromBytes for StoredProjMeta {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(bytes);

        // --- TRACK METAS ---

        // Read the length of the track metadatas
        let mut track_metas_len_bytes = [0u8; 8];
        cursor.read_exact(&mut track_metas_len_bytes)?;
        let track_metas_len = u64::from_le_bytes(track_metas_len_bytes);

        // Read the track metadatas
        let track_metas_bytes = safe_read(&mut cursor, track_metas_len as usize)?;

        // Loop over the track metadatas and read each track metadata
        let mut track_metas = HashMap::new();
        let mut track_metas_cursor = Cursor::new(track_metas_bytes.as_slice());
        while track_metas_cursor.position() < track_metas_len {
            // Read the track ID and the length of the track metadata
            let mut track_id_bytes = [0u8; 8];
            track_metas_cursor.read_exact(&mut track_id_bytes)?;
            let track_id = TrackID(u64::from_le_bytes(track_id_bytes) as usize);

            let mut track_meta_len_bytes = [0u8; 8];
            track_metas_cursor.read_exact(&mut track_meta_len_bytes)?;
            let track_meta_len = u64::from_le_bytes(track_meta_len_bytes) as usize;

            // Read the track metadata
            let track_meta_bytes = safe_read(&mut track_metas_cursor, track_meta_len)?;

            // Convert the track metadata bytes to a StoredTrackMeta and insert it into the HashMap
            let track_meta = StoredTrackMeta::from_bytes(&track_meta_bytes)?;
            track_metas.insert(track_id, track_meta);
        }

        // --- SEARCH PATHS ---

        // Read the length of the kasl search paths
        let mut kasl_search_paths_len_bytes = [0u8; 8];
        cursor.read_exact(&mut kasl_search_paths_len_bytes)?;
        let kasl_search_paths_len = u64::from_le_bytes(kasl_search_paths_len_bytes);

        // Read the kasl search paths data
        let kasl_search_paths_bytes = safe_read(&mut cursor, kasl_search_paths_len as usize)?;

        // Read the kasl search paths
        let mut kasl_search_paths = Vec::new();
        let mut kasl_search_paths_cursor = Cursor::new(kasl_search_paths_bytes.as_slice());
        while kasl_search_paths_cursor.position() < kasl_search_paths_len {
            // Read the length of the path
            let mut path_len_bytes = [0u8; 8];
            kasl_search_paths_cursor.read_exact(&mut path_len_bytes)?;
            let path_len = u64::from_le_bytes(path_len_bytes) as usize;

            // Read the path bytes and convert it to a String
            let path_bytes = safe_read(&mut kasl_search_paths_cursor, path_len)?;
            let path = String::from_utf8(path_bytes)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            kasl_search_paths.push(path);
        }

        // Construct the StoredProjMeta
        let proj_meta = StoredProjMeta {
            track_metas,
            kasl_search_paths,
        };

        Ok(proj_meta)
    }
}
