use crate::ui::EditorUi;
use kreniq_engine::{graph::node_id::NodeID, mixer::TrackID};

impl EditorUi {
    pub(crate) fn remove_edge(&mut self, track_id: &TrackID, edge: (NodeID, usize, NodeID, usize)) {
        if let Some(track) = self.project.get_track_mut(track_id) {
            if let Err(err) = track.get_graph_mut().remove_edge(edge) {
                eprintln!("Failed to remove edge: {:#?}", err);
                return;
            };

            // Update the project on the audio thread
            self.modified_project();
        }
    }

    pub(crate) fn add_edge(&mut self, track_id: &TrackID, edge: (NodeID, usize, NodeID, usize)) {
        if let Some(track) = self.project.get_track_mut(track_id) {
            if let Err(err) = track.get_graph_mut().add_edge(edge) {
                eprintln!("Failed to add edge: {:#?}", err);
                return;
            };

            // Update the project on the audio thread
            self.modified_project();
        }
    }
}
