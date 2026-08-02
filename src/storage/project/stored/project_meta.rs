use crate::{core::metadata::ProjectMeta, storage::project::stored::track_meta::StoredTrackMeta};
use kadent_engine::mixer::TrackID;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub(crate) struct StoredProjMeta {
    pub track_metas: HashMap<TrackID, StoredTrackMeta>,
    pub kasl_search_paths: Vec<String>,
}

impl StoredProjMeta {
    pub fn from_project_meta(project_meta: &ProjectMeta) -> Self {
        let track_metas = project_meta
            .tracks
            .iter()
            .map(|(track_id, track_meta)| (*track_id, StoredTrackMeta::from_track_meta(track_meta)))
            .collect();

        Self {
            track_metas,
            kasl_search_paths: project_meta.kasl_search_paths.clone(),
        }
    }
}
