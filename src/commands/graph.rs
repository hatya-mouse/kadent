use knodiq_engine::{graph::node_id::NodeID, mixer::TrackID};

use crate::ui::EditorUi;

impl EditorUi {
    pub(crate) fn remove_edge(&mut self, track_id: &TrackID, edge: (NodeID, usize, NodeID, usize)) {
        if let Some(track) = self.project.get_track_mut(track_id) {
            track.get_graph_mut().remove_edge(edge);

            // Update the project on the audio thread
            self.modified_project();
        }
    }

    pub(crate) fn add_edge(&mut self, track_id: &TrackID, edge: (NodeID, usize, NodeID, usize)) {
        if let Some(track) = self.project.get_track_mut(track_id) {
            track.get_graph_mut().add_edge(edge);

            // Update the project on the audio thread
            self.modified_project();
        }
    }
}
