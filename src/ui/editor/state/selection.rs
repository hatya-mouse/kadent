use crate::core::audio_engine::{
    graph::node_id::NodeID,
    mixer::TrackID,
    track::{RegionID, note_track::NoteID},
};

#[derive(Default)]
pub(crate) enum Selection {
    #[default]
    None,
    Track(TrackID),
    Region(TrackID, RegionID),
    Node(TrackID, NodeID),
    Keyframe(TrackID, NodeID, usize),
    Note(TrackID, RegionID, NoteID),
}

impl Selection {
    // --- MUTATION ---

    pub(crate) fn select_track(&mut self, track_id: TrackID) {
        *self = Selection::Track(track_id);
    }

    pub(crate) fn select_region(&mut self, track_id: TrackID, region_id: RegionID) {
        *self = Selection::Region(track_id, region_id);
    }

    pub(crate) fn select_node(&mut self, track_id: TrackID, node_id: NodeID) {
        *self = Selection::Node(track_id, node_id);
    }

    pub(crate) fn select_keyframe(
        &mut self,
        track_id: TrackID,
        node_id: NodeID,
        keyframe_index: usize,
    ) {
        *self = Selection::Keyframe(track_id, node_id, keyframe_index);
    }

    pub(crate) fn select_note(&mut self, track_id: TrackID, region_id: RegionID, note_id: NoteID) {
        *self = Selection::Note(track_id, region_id, note_id);
    }

    pub(crate) fn clear(&mut self) {
        *self = Selection::None;
    }

    // --- GETTERS ---

    pub(crate) fn track_id(&self) -> Option<TrackID> {
        match self {
            Selection::Track(track_id) => Some(*track_id),
            Selection::Region(track_id, _) => Some(*track_id),
            Selection::Node(track_id, _) => Some(*track_id),
            Selection::Keyframe(track_id, _, _) => Some(*track_id),
            Selection::Note(track_id, _, _) => Some(*track_id),
            Selection::None => None,
        }
    }

    pub(crate) fn region_id(&self) -> Option<RegionID> {
        match self {
            Selection::Region(_, region_id) => Some(*region_id),
            Selection::Note(_, region_id, _) => Some(*region_id),
            _ => None,
        }
    }

    pub(crate) fn note_id(&self) -> Option<NoteID> {
        match self {
            Selection::Note(_, _, note_id) => Some(*note_id),
            _ => None,
        }
    }

    pub(crate) fn node_id(&self) -> Option<NodeID> {
        match self {
            Selection::Node(_, node_id) => Some(*node_id),
            Selection::Keyframe(_, node_id, _) => Some(*node_id),
            _ => None,
        }
    }

    pub(crate) fn track_and_region_id(&self) -> Option<(TrackID, RegionID)> {
        match self {
            Selection::Region(track_id, region_id) => Some((*track_id, *region_id)),
            Selection::Note(track_id, region_id, _) => Some((*track_id, *region_id)),
            _ => None,
        }
    }

    pub(crate) fn track_and_node_id(&self) -> Option<(TrackID, NodeID)> {
        match self {
            Selection::Node(track_id, node_id) => Some((*track_id, *node_id)),
            Selection::Keyframe(track_id, node_id, _) => Some((*track_id, *node_id)),
            _ => None,
        }
    }

    pub(crate) fn keyframe_index(&self) -> Option<usize> {
        match self {
            Selection::Keyframe(_, _, index) => Some(*index),
            _ => None,
        }
    }
}
