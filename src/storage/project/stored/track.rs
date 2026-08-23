use crate::core::audio_engine::track::{
    RegionID, Track,
    audio_track::{AudioRegion, AudioTrack},
    note_track::{NoteRegion, NoteTrack},
};
use crate::storage::project::stored::graph::StoredGraph;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Mirror of `Box<dyn Track>` for persistence.
#[derive(Serialize, Deserialize)]
pub(crate) enum StoredTrack {
    Audio(StoredAudioTrack),
    Note(StoredNoteTrack),
}

impl StoredTrack {
    /// Returns `None` if the track is of an unrecognized type.
    pub(crate) fn from_track(track: &dyn Track) -> Option<Self> {
        if let Some(audio_track) = track.as_any().downcast_ref::<AudioTrack>() {
            Some(Self::Audio(StoredAudioTrack::from_audio_track(audio_track)))
        } else {
            track
                .as_any()
                .downcast_ref::<NoteTrack>()
                .map(|note_track| Self::Note(StoredNoteTrack::from_note_track(note_track)))
        }
    }

    pub(crate) fn to_track(&self) -> Box<dyn Track> {
        match self {
            Self::Audio(stored) => Box::new(stored.to_audio_track()),
            Self::Note(stored) => Box::new(stored.to_note_track()),
        }
    }
}

/// Mirror of `AudioTrack`. `pre_processed`/`local_buffer`/`audio_ctx` aren't stored
/// and they're rebuilt by `prepare()`/`set_audio_ctx` after load.
/// `next_region_id` is recomputed from the loaded region IDs.
#[derive(Serialize, Deserialize)]
pub(crate) struct StoredAudioTrack {
    pub(crate) graph: StoredGraph,
    pub(crate) regions: HashMap<RegionID, AudioRegion>,
}

impl StoredAudioTrack {
    pub(crate) fn from_audio_track(track: &AudioTrack) -> Self {
        Self {
            graph: StoredGraph::from_graph(track.get_graph()),
            regions: track.get_all_regions().clone(),
        }
    }

    pub(crate) fn to_audio_track(&self) -> AudioTrack {
        let mut track = AudioTrack::new();
        track.set_graph(self.graph.to_graph());
        track.set_regions(self.regions.clone());
        restore_next_region_id(&mut track);
        track
    }
}

fn restore_next_region_id(track: &mut AudioTrack) {
    let next_id = track
        .get_all_regions()
        .keys()
        .map(|id| id.0)
        .max()
        .map(|m| m + 1)
        .unwrap_or(0);
    track.set_next_region_id(next_id);
}

/// Mirror of `NoteTrack`.
#[derive(Serialize, Deserialize)]
pub(crate) struct StoredNoteTrack {
    pub(crate) graph: StoredGraph,
    pub(crate) regions: HashMap<RegionID, NoteRegion>,
}

impl StoredNoteTrack {
    pub(crate) fn from_note_track(track: &NoteTrack) -> Self {
        Self {
            graph: StoredGraph::from_graph(track.get_graph()),
            regions: track.get_all_regions().clone(),
        }
    }

    pub(crate) fn to_note_track(&self) -> NoteTrack {
        let mut track = NoteTrack::new();
        track.set_graph(self.graph.to_graph());
        track.set_regions(self.regions.clone());
        restore_next_region_id_note(&mut track);
        track
    }
}

fn restore_next_region_id_note(track: &mut NoteTrack) {
    let next_id = track
        .get_all_regions()
        .keys()
        .map(|id| id.0)
        .max()
        .map(|m| m + 1)
        .unwrap_or(0);
    track.set_next_region_id(next_id);
}
