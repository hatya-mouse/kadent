use crate::{
    load_write::StoredTrackMeta,
    metadata::{GraphMeta, RegionMeta},
    ui_state::dialog_state::TrackType,
};
use eframe::egui;
use kreniq_engine::track::{RegionID, Track, audio_track::AudioTrack, note_track::NoteTrack};
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct TrackMeta {
    pub name: String,
    pub color: egui::Color32,
    pub track_type: TrackType,
    pub regions: HashMap<RegionID, RegionMeta>,
    pub graph: GraphMeta,
}

impl TrackMeta {
    pub fn new(name: String, color: egui::Color32, track_type: TrackType) -> Self {
        Self {
            name,
            color,
            track_type,
            regions: HashMap::new(),
            graph: GraphMeta::default(),
        }
    }

    pub fn from_stored(track: &dyn Track, track_meta: &StoredTrackMeta) -> Self {
        // Determine track type based on the track's type
        if let Some(audio_track) = track.as_any().downcast_ref::<AudioTrack>() {
            // Get the audio regions from the track
            let mut regions = HashMap::new();
            for (region_id, audio_region) in audio_track.get_all_regions() {
                let Some(stored_region_meta) = track_meta.region_metas.get(region_id) else {
                    continue;
                };

                regions.insert(
                    *region_id,
                    RegionMeta::new(
                        stored_region_meta.name.clone(),
                        audio_region.start,
                        audio_region.duration,
                        None,
                    ),
                );
            }

            Self {
                name: track_meta.name.clone(),
                color: track_meta.color,
                track_type: TrackType::Audio,
                regions,
                graph: track_meta.node_graph.to_graph_meta(),
            }
        } else if let Some(note_track) = track.as_any().downcast_ref::<NoteTrack>() {
            // Get the note regions from the track
            let mut regions = HashMap::new();
            for (region_id, audio_region) in note_track.get_all_regions() {
                let Some(stored_region_meta) = track_meta.region_metas.get(region_id) else {
                    continue;
                };

                regions.insert(
                    *region_id,
                    RegionMeta::new(
                        stored_region_meta.name.clone(),
                        audio_region.start,
                        audio_region.duration,
                        None,
                    ),
                );
            }

            Self {
                name: track_meta.name.clone(),
                color: track_meta.color,
                track_type: TrackType::Note,
                regions,
                graph: track_meta.node_graph.to_graph_meta(),
            }
        } else {
            unreachable!("There must be no tracks other than AudioTrack and NoteTrack");
        }
    }

    // --- REGION MANAGEMENT ---

    pub fn add_region(&mut self, id: RegionID, region: RegionMeta) {
        self.regions.insert(id, region);
    }

    pub fn get_region(&self, id: &RegionID) -> Option<&RegionMeta> {
        self.regions.get(id)
    }

    pub fn get_region_mut(&mut self, id: &RegionID) -> Option<&mut RegionMeta> {
        self.regions.get_mut(id)
    }

    pub fn remove_region(&mut self, id: &RegionID) {
        self.regions.remove(id);
    }
}
