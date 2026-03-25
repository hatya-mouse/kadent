mod track_meta;

pub(crate) use track_meta::TrackMeta;

use knodiq_engine::mixer::TrackID;
use std::collections::HashMap;

pub(crate) struct ProjectMeta {
    pub tracks: HashMap<TrackID, TrackMeta>,
    pub track_order: Vec<TrackID>,
    pub name: String,
}

impl ProjectMeta {
    pub fn new(name: String) -> Self {
        Self {
            tracks: HashMap::new(),
            track_order: Vec::new(),
            name,
        }
    }
}
