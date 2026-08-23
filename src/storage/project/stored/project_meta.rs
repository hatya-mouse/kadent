use crate::core::audio_engine::{data_types::PlaybackContext, mixer::TrackID};
use crate::{core::metadata::ProjectMeta, storage::project::stored::track_meta::StoredTrackMeta};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub(crate) struct StoredProjMeta {
    pub(crate) track_metas: HashMap<TrackID, StoredTrackMeta>,
    pub(crate) kasl_search_paths: Vec<String>,
    pub(crate) export_ctx: PlaybackContext,
}

impl StoredProjMeta {
    pub(crate) fn from_project_meta(project_meta: &ProjectMeta) -> Self {
        let track_metas = project_meta
            .tracks
            .iter()
            .map(|(track_id, track_meta)| (*track_id, StoredTrackMeta::from_track_meta(track_meta)))
            .collect();

        Self {
            track_metas,
            kasl_search_paths: project_meta.kasl_search_paths.clone(),
            export_ctx: project_meta.export_ctx.clone(),
        }
    }
}
