use crate::storage::project::{
    AsBytes, FromBytes,
    error::{Contextualize, LoadError, ParseContext},
    trait_impl::project_meta::{StoredGraphMeta, StoredRegionMeta},
    traits::safe_read,
};
use eframe::egui;
use kadent_engine::track::RegionID;
use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

pub struct StoredTrackMeta {
    pub name: String,
    pub color: egui::Color32,
    pub region_metas: HashMap<RegionID, StoredRegionMeta>,
    pub node_graph: StoredGraphMeta,
}

impl StoredTrackMeta {
    pub fn from_track_meta(track_meta: &crate::core::metadata::TrackMeta) -> Self {
        let region_metas = track_meta
            .regions
            .iter()
            .map(|(region_id, region_meta)| {
                (*region_id, StoredRegionMeta::from_region_meta(region_meta))
            })
            .collect();

        Self {
            name: track_meta.name.clone(),
            color: track_meta.color,
            region_metas,
            node_graph: StoredGraphMeta::from_graph_meta(&track_meta.graph),
        }
    }
}

impl AsBytes for StoredTrackMeta {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write the color of the track to the bytes vector
        bytes.push(self.color.r());
        bytes.push(self.color.g());
        bytes.push(self.color.b());
        bytes.push(self.color.a());

        // Write the length of the name, and the name itself
        let name_bytes = self.name.as_bytes();
        bytes.extend((name_bytes.len() as u64).to_le_bytes());
        bytes.extend(name_bytes);

        // Write the region metadatas
        let mut region_metas_bytes = Vec::new();
        for (region_id, region_meta) in &self.region_metas {
            // Get the length of the region metadata
            let mut region_meta_bytes = Vec::new();
            region_meta.as_bytes(&mut region_meta_bytes);

            // Write the region ID, the length of the region metadata, and the region metadata itself
            region_metas_bytes.extend(region_id.0.to_le_bytes());
            region_metas_bytes.extend((region_meta_bytes.len() as u64).to_le_bytes());
            region_metas_bytes.extend(region_meta_bytes);
        }

        // Write the length of the region metadatas, and the region metadatas themselves
        bytes.extend((region_metas_bytes.len() as u64).to_le_bytes());
        bytes.extend(region_metas_bytes);

        // Write the node graph layout
        let mut node_graph_bytes = Vec::new();
        self.node_graph.as_bytes(&mut node_graph_bytes);
        bytes.extend((node_graph_bytes.len() as u64).to_le_bytes());
        bytes.extend(node_graph_bytes);
    }
}

impl FromBytes for StoredTrackMeta {
    fn from_bytes(bytes: &[u8]) -> Result<Self, LoadError> {
        let mut cursor = Cursor::new(bytes);

        // Read the first four bytes to get the color of the track
        let mut color_bytes = [0u8; 4];
        cursor
            .read_exact(&mut color_bytes)
            .with_ctx(ParseContext::TrackMeta)?;
        let color = egui::Color32::from_rgba_premultiplied(
            color_bytes[0],
            color_bytes[1],
            color_bytes[2],
            color_bytes[3],
        );

        // Read the length of the name
        let mut name_len_bytes = [0u8; 8];
        cursor
            .read_exact(&mut name_len_bytes)
            .with_ctx(ParseContext::TrackMeta)?;
        let name_len = u64::from_le_bytes(name_len_bytes) as usize;
        // Read the name itself
        let name_bytes = safe_read(&mut cursor, name_len).with_ctx(ParseContext::TrackMeta)?;
        let name = String::from_utf8(name_bytes)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            .with_ctx(ParseContext::TrackMeta)?;

        // Read the length of the region metadatas
        let mut region_metas_len_bytes = [0u8; 8];
        cursor
            .read_exact(&mut region_metas_len_bytes)
            .with_ctx(ParseContext::TrackMeta)?;
        let region_metas_len = u64::from_le_bytes(region_metas_len_bytes);

        // Read the region metadatas
        let region_metas_bytes =
            safe_read(&mut cursor, region_metas_len as usize).with_ctx(ParseContext::TrackMeta)?;

        let mut region_metas = HashMap::new();
        let mut region_metas_cursor = Cursor::new(region_metas_bytes.as_slice());
        while region_metas_cursor.position() < region_metas_len {
            // Read the region ID
            let mut region_id_bytes = [0u8; 8];
            region_metas_cursor
                .read_exact(&mut region_id_bytes)
                .with_ctx(ParseContext::TrackMeta)?;
            let region_id = RegionID(u64::from_le_bytes(region_id_bytes) as usize);

            // Read the length of the region metadata
            let mut region_meta_len_bytes = [0u8; 8];
            region_metas_cursor
                .read_exact(&mut region_meta_len_bytes)
                .with_ctx(ParseContext::TrackMeta)?;
            let region_meta_len = u64::from_le_bytes(region_meta_len_bytes) as usize;

            // Read the region metadata itself
            let region_meta_bytes = safe_read(&mut region_metas_cursor, region_meta_len)
                .with_ctx(ParseContext::TrackMeta)?;
            let region_meta = StoredRegionMeta::from_bytes(&region_meta_bytes)?;

            // Inser the region metadata into the hashmap
            region_metas.insert(region_id, region_meta);
        }

        // Read the node graph layout (falls back to default for older project files)
        let node_graph = read_node_graph_layout(&mut cursor).unwrap_or_default();

        Ok(Self {
            name,
            color,
            region_metas,
            node_graph,
        })
    }
}

fn read_node_graph_layout(cursor: &mut Cursor<&[u8]>) -> Result<StoredGraphMeta, LoadError> {
    let mut len_bytes = [0u8; 8];
    cursor
        .read_exact(&mut len_bytes)
        .with_ctx(ParseContext::TrackMeta)?;
    let len = u64::from_le_bytes(len_bytes) as usize;
    let layout_bytes = safe_read(cursor, len).with_ctx(ParseContext::TrackMeta)?;
    StoredGraphMeta::from_bytes(&layout_bytes)
}
