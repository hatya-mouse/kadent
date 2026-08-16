use crate::ui::EditorUi;
use kadent_engine::{graph::node_id::NodeID, mixer::TrackID};

impl EditorUi {
    pub(in crate::actions) fn remove_edge(&mut self, track_id: &TrackID, to: &(NodeID, usize)) {
        if let Some(track) = self.ui_state.proj_ctx.project.get_track_mut(track_id) {
            track.get_graph_mut().remove_edge(to);
            // Update the project on the audio thread
            self.modified_project();
        }
    }

    pub(in crate::actions) fn add_edge(
        &mut self,
        track_id: &TrackID,
        from: (NodeID, usize),
        to: (NodeID, usize),
    ) {
        if let Some(track) = self.ui_state.proj_ctx.project.get_track_mut(track_id) {
            if let Err(err) = track.get_graph_mut().add_edge(from, to) {
                eprintln!("Failed to add edge: {:#?}", err);
                return;
            };

            // Update the project on the audio thread
            self.modified_project();
        }
    }
}
