use crate::ui::EditorState;
use eframe::egui;
use kadent_engine::{graph::node_id::NodeID, mixer::TrackID, node::builtin::AutomationNode};

impl EditorState {
    pub(super) fn keyframe_inspector(
        &mut self,
        ui: &mut egui::Ui,
        track_id: &TrackID,
        node_id: &NodeID,
        keyframe_index: usize,
    ) {
        let Some(track) = self
            .project
            .data
            .get_track_mut(track_id)
            .and_then(|track| track.get_graph_mut().get_node_mut(node_id))
            .and_then(|node| node.as_any_mut().downcast_mut::<AutomationNode>())
            .map(|node| &mut node.track)
        else {
            return;
        };
    }
}
