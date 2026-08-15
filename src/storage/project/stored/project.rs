use crate::storage::project::stored::{tempo_map::StoredTempoMap, track::StoredTrack};
use kadent_engine::{
    data_types::AudioContext,
    mixer::{Project, TrackID},
    timing::TimeBounds,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Mirror of `Project` for persistence. `next_track_id` isn't stored
/// and it's recomputed from the loaded track IDs.
#[derive(Serialize, Deserialize)]
pub(crate) struct StoredProject {
    pub tracks: HashMap<TrackID, StoredTrack>,
    pub tempo_map: StoredTempoMap,
    pub audio_ctx: AudioContext,
    pub export_range: TimeBounds,
}

impl StoredProject {
    pub fn from_project(project: &Project) -> Self {
        let tracks = project
            .tracks
            .iter()
            .filter_map(|(id, track)| {
                StoredTrack::from_track(track.as_ref()).map(|stored| (*id, stored))
            })
            .collect();

        Self {
            tracks,
            tempo_map: StoredTempoMap::from_tempo_map(&project.tempo_map),
            audio_ctx: project.audio_ctx.clone(),
            export_range: project.export_range.clone(),
        }
    }

    pub fn to_project(&self) -> Project {
        let tempo_map = self.tempo_map.to_tempo_map(&self.audio_ctx);
        let mut project =
            Project::with_tempo_map(self.audio_ctx.clone(), tempo_map, self.export_range.clone());

        for (id, stored_track) in &self.tracks {
            project.tracks.insert(*id, stored_track.to_track());
        }
        restore_next_track_id(&mut project);

        project
    }
}

fn restore_next_track_id(project: &mut Project) {
    let next_id = project
        .tracks
        .keys()
        .map(|id| id.0)
        .max()
        .map(|m| m + 1)
        .unwrap_or(0);
    project.set_next_track_id(next_id);
}
