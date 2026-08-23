use crate::core::audio_engine::track::RegionID;
use crate::{
    core::metadata::TrackMeta,
    storage::project::stored::{graph_meta::StoredGraphMeta, region_meta::StoredRegionMeta},
};
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub(crate) struct StoredTrackMeta {
    pub(crate) name: String,
    /// (r, g, b, a). `egui::Color32` doesn't derive `Serialize`, so it's stored as a plain tuple.
    pub(crate) color: (u8, u8, u8, u8),
    pub(crate) region_metas: HashMap<RegionID, StoredRegionMeta>,
    pub(crate) node_graph: StoredGraphMeta,
}

impl StoredTrackMeta {
    pub(crate) fn from_track_meta(track_meta: &TrackMeta) -> Self {
        let region_metas = track_meta
            .regions
            .iter()
            .map(|(region_id, region_meta)| {
                (*region_id, StoredRegionMeta::from_region_meta(region_meta))
            })
            .collect();

        Self {
            name: track_meta.name.clone(),
            color: (
                track_meta.color.r(),
                track_meta.color.g(),
                track_meta.color.b(),
                track_meta.color.a(),
            ),
            region_metas,
            node_graph: StoredGraphMeta::from_graph_meta(&track_meta.graph),
        }
    }

    pub(crate) fn color(&self) -> egui::Color32 {
        egui::Color32::from_rgba_premultiplied(
            self.color.0,
            self.color.1,
            self.color.2,
            self.color.3,
        )
    }
}
