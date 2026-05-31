use crate::ui::EditorUi;
use kreniq_engine::{graph::node_id::NodeID, mixer::TrackID};

impl EditorUi {
    pub(crate) fn remove_node(&mut self, track_id: &TrackID, node_id: &NodeID) {
        // Get the track and track metadata
        let Some(track) = self.project.get_track_mut(track_id) else {
            return;
        };
        let Some(track_meta) = self.project_meta.get_track_mut(track_id) else {
            return;
        };

        // Check if the node is not an input or an output node
        if &track.get_graph().get_input_id() == node_id
            || &track.get_graph().get_output_id() == node_id
        {
            return;
        }

        // Remove the node from the track and the track metadata
        track.get_graph_mut().remove_node(node_id);
        track_meta.graph.remove_node(node_id);

        self.ui_state.deselect_node();

        // Update the project
        self.modified_project();
    }
}
