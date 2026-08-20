use kadent_engine::{
    graph::node_id::NodeID,
    mixer::TrackID,
    node::builtin::{AutomationNode, AutomationTarget, Keyframe},
};

use crate::ui::EditorState;

impl EditorState {
    pub(super) fn add_keyframe<T>(
        &mut self,
        track_id: &TrackID,
        node_id: &NodeID,
        keyframe: Keyframe<T>,
    ) where
        T: AutomationTarget,
    {
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

        let keyframe_index = track.add_keyframe(keyframe);
        self.selection
            .select_keyframe(*track_id, *node_id, keyframe_index);
        self.modified_project();
    }

    pub(super) fn remove_keyframe(
        &mut self,
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

        track.remove_keyframe(keyframe_index);
        self.modified_project();
    }
}
