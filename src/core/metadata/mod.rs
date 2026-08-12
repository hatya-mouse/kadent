mod graph_meta;
mod node_meta;
mod region_meta;
mod track_meta;

pub(crate) use graph_meta::GraphMeta;
pub(crate) use node_meta::{NodeMeta, NodeType};
pub(crate) use region_meta::RegionMeta;
pub(crate) use track_meta::{TrackMeta, TrackType};

use crate::{
    consts::{DEFAULT_BUFFER_SIZE, DEFAULT_CHANNELS, DEFAULT_SAMPLE_RATE},
    storage::project::LoadProjResult,
};
use kadent_engine::{
    data_types::{PlaybackContext, Ticks},
    mixer::TrackID,
};
use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub(crate) struct ProjectMeta {
    pub tracks: HashMap<TrackID, TrackMeta>,
    pub track_order: Vec<TrackID>,
    pub kasl_search_paths: Vec<String>,
    pub range_start: Ticks,
    pub range_duration: Ticks,
    pub export_ctx: PlaybackContext,
}

#[derive(Debug)]
pub enum ProjectMetaLoadingError {
    MissingTrackMeta(TrackID),
}

impl ProjectMeta {
    pub fn from_load_res(proj_res: &LoadProjResult) -> Result<Self, ProjectMetaLoadingError> {
        let export_ctx = proj_res.project_meta.export_ctx.clone();
        // If the export context is corrupted, use default values instead
        let export_ctx = if export_ctx.channels == 0
            || export_ctx.sample_rate == 0
            || export_ctx.buffer_size == 0
        {
            PlaybackContext {
                channels: DEFAULT_CHANNELS,
                sample_rate: DEFAULT_SAMPLE_RATE,
                buffer_size: DEFAULT_BUFFER_SIZE,
            }
        } else {
            export_ctx
        };

        let mut new_meta = ProjectMeta {
            kasl_search_paths: proj_res.project_meta.kasl_search_paths.clone(),
            range_start: proj_res.project.range_start,
            range_duration: proj_res.project.range_duration,
            export_ctx,
            ..Default::default()
        };

        // Initialize the tracks
        for (track_id, track) in &proj_res.project.tracks {
            if let Some(stored_track_meta) = proj_res.project_meta.track_metas.get(track_id) {
                let track_meta = TrackMeta::from_stored(track.as_ref(), stored_track_meta);
                new_meta.add_track(*track_id, track_meta);
            } else {
                return Err(ProjectMetaLoadingError::MissingTrackMeta(*track_id));
            }
        }

        Ok(new_meta)
    }

    // --- TRACK MANAGEMENT ---

    /// Adds a new track to the project with the given ID.
    pub fn add_track(&mut self, id: TrackID, track: TrackMeta) {
        self.tracks.insert(id, track);
        self.track_order.push(id);
    }

    /// Removes a track from the project with the given ID.
    pub fn remove_track(&mut self, id: &TrackID) {
        self.tracks.remove(id);
        self.track_order.retain(|&track_id| track_id != *id);
    }

    /// Returns a reference to the track with the given ID.
    pub fn get_track(&self, id: &TrackID) -> Option<&TrackMeta> {
        self.tracks.get(id)
    }

    /// Returns a mutable reference to the track with the given ID.
    pub fn get_track_mut(&mut self, id: &TrackID) -> Option<&mut TrackMeta> {
        self.tracks.get_mut(id)
    }
}
