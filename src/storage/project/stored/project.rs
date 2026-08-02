use crate::storage::project::stored::{tempo_map::StoredTempoMap, track::StoredTrack};
use kadent_engine::{
    data_types::{AudioContext, Ticks},
    mixer::{Project, TrackID},
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
    pub range_start: Ticks,
    pub range_duration: Ticks,
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
            range_start: project.range_start,
            range_duration: project.range_duration,
        }
    }

    pub fn to_project(&self) -> Project {
        let tempo_map = self.tempo_map.to_tempo_map(&self.audio_ctx);
        let mut project = Project::with_tempo_map(
            self.audio_ctx.clone(),
            tempo_map,
            self.range_start,
            self.range_duration,
        );

        for (id, stored_track) in &self.tracks {
            project
                .tracks
                .insert(*id, stored_track.to_track(&self.audio_ctx));
        }
        restore_next_track_id(&mut project);

        // Restore the audio context on every track
        let audio_ctx = self.audio_ctx.clone();
        for track in project.tracks.values_mut() {
            track.set_audio_ctx(&audio_ctx);
        }

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
