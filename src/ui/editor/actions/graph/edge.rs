use crate::core::audio_engine::{graph::node_id::NodeID, mixer::TrackID};
use crate::ui::EditorState;

impl EditorState {
    pub(crate) fn remove_edge(&mut self, track_id: &TrackID, to: &(NodeID, usize)) {
        if let Some(track) = self.project.data.get_track_mut(track_id) {
            track.get_graph_mut().remove_edge(to);
            // Update the project on the audio thread
            self.modified_project();
        }
    }

    pub(crate) fn add_edge(
        &mut self,
        track_id: &TrackID,
        from: (NodeID, usize),
        to: (NodeID, usize),
    ) {
        if let Some(track) = self.project.data.get_track_mut(track_id) {
            if let Err(err) = track.get_graph_mut().add_edge(from, to) {
                eprintln!("Failed to add edge: {:#?}", err);
                return;
            };

            // Update the project on the audio thread
            self.modified_project();
        }
    }
}
