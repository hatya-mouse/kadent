use crate::core::audio_engine::track::RegionID;
use crate::core::metadata::{GraphMeta, RegionMeta};
use eframe::egui;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub(crate) struct TrackMeta {
    pub(crate) name: String,
    pub(crate) color: egui::Color32,
    pub(crate) track_type: TrackType,
    pub(crate) regions: HashMap<RegionID, RegionMeta>,
    pub(crate) graph: GraphMeta,
}

#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(u32)]
pub(crate) enum TrackType {
    Audio = 0,
    Note = 1,
}

impl TrackType {
    pub(crate) fn all() -> [Self; 2] {
        [Self::Audio, Self::Note]
    }

    pub(crate) fn fmt_lowercase(&self) -> String {
        match self {
            TrackType::Audio => "audio".to_string(),
            TrackType::Note => "note".to_string(),
        }
    }
}

impl Display for TrackType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrackType::Audio => write!(f, "Audio"),
            TrackType::Note => write!(f, "Note"),
        }
    }
}

impl TrackMeta {
    pub(crate) fn new(name: String, color: egui::Color32, track_type: TrackType) -> Self {
        Self {
            name,
            color,
            track_type,
            regions: HashMap::new(),
            graph: GraphMeta::default(),
        }
    }

    // --- REGION MANAGEMENT ---

    pub(crate) fn add_region(&mut self, id: RegionID, region: RegionMeta) {
        self.regions.insert(id, region);
    }

    pub(crate) fn get_region(&self, id: &RegionID) -> Option<&RegionMeta> {
        self.regions.get(id)
    }

    pub(crate) fn get_region_mut(&mut self, id: &RegionID) -> Option<&mut RegionMeta> {
        self.regions.get_mut(id)
    }

    pub(crate) fn remove_region(&mut self, id: &RegionID) -> Option<RegionMeta> {
        self.regions.remove(id)
    }
}
