use crate::ui::{
    EditorState,
    editor::actions::{KeyframeType, KeyframeValue},
};
use kadent_engine::{
    graph::node_id::NodeID,
    mixer::TrackID,
    node::builtin::{AutomationNode, AutomationTrack, AutomationTrackType, CurveType},
};

impl EditorState {
    fn get_automation_node_mut(
        &mut self,
        track_id: &TrackID,
        node_id: &NodeID,
    ) -> Option<&mut AutomationNode> {
        self.project
            .data
            .get_track_mut(track_id)
            .and_then(|track| track.get_graph_mut().get_node_mut(node_id))
            .and_then(|node| node.as_any_mut().downcast_mut::<AutomationNode>())
    }

    pub(super) fn add_keyframe(
        &mut self,
        track_id: &TrackID,
        node_id: &NodeID,
        keyframe: KeyframeType,
    ) {
        if let Some(node) = self.get_automation_node_mut(track_id, node_id) {
            let keyframe_index = match keyframe {
                KeyframeType::Float(keyframe) => node.track.add_float_keyframe(keyframe),
                KeyframeType::Int(keyframe) => node.track.add_int_keyframe(keyframe),
                KeyframeType::Bool(keyframe) => node.track.add_bool_keyframe(keyframe),
            };
            self.selection
                .select_keyframe(*track_id, *node_id, keyframe_index);
            self.modified_project();
        }
    }

    pub(super) fn remove_keyframe(
        &mut self,
        track_id: &TrackID,
        node_id: &NodeID,
        keyframe_index: usize,
    ) {
        if let Some(node) = self.get_automation_node_mut(track_id, node_id) {
            node.track.remove_keyframe(keyframe_index);
            self.modified_project();
        }
    }

    pub(super) fn set_keyframe_value(
        &mut self,
        track_id: &TrackID,
        node_id: &NodeID,
        keyframe_index: usize,
        new_value: KeyframeValue,
    ) {
        if let Some(node) = self.get_automation_node_mut(track_id, node_id) {
            match new_value {
                KeyframeValue::Float(value) => node.track.set_float_value(keyframe_index, value),
                KeyframeValue::Int(value) => node.track.set_int_value(keyframe_index, value),
                KeyframeValue::Bool(value) => node.track.set_bool_value(keyframe_index, value),
            }
            self.modified_project();
        }
    }

    pub(super) fn set_keyframe_curve_type(
        &mut self,
        track_id: &TrackID,
        node_id: &NodeID,
        keyframe_index: usize,
        new_curve: CurveType,
    ) {
        if let Some(node) = self.get_automation_node_mut(track_id, node_id) {
            node.track.set_curve_type(keyframe_index, new_curve);
            self.modified_project();
        }
    }

    pub(super) fn set_automation_type(
        &mut self,
        track_id: &TrackID,
        node_id: &NodeID,
        new_type: AutomationTrackType,
    ) {
        if let Some(node) = self.get_automation_node_mut(track_id, node_id) {
            match new_type {
                AutomationTrackType::Float => {
                    node.track = AutomationTrack::new_float(0.0..=1.0);
                }
                AutomationTrackType::Int => {
                    node.track = AutomationTrack::new_int(0..=100);
                }
                AutomationTrackType::Bool => {
                    node.track = AutomationTrack::new_bool();
                }
            }
            self.modified_project();
        }
    }

    pub(super) fn set_automation_max_value(
        &mut self,
        track_id: &TrackID,
        node_id: &NodeID,
        max_value: KeyframeValue,
    ) {
        if let Some(node) = self.get_automation_node_mut(track_id, node_id) {
            match &mut node.track {
                AutomationTrack::Float { range, .. } => {
                    if let KeyframeValue::Float(max_value) = max_value {
                        *range = *range.start()..=max_value.max(*range.start());
                        self.modified_project();
                    }
                }
                AutomationTrack::Int { range, .. } => {
                    if let KeyframeValue::Int(max_value) = max_value {
                        *range = *range.start()..=max_value.max(*range.start());
                        self.modified_project();
                    }
                }
                AutomationTrack::Bool { .. } => (),
            }
        }
    }

    pub(super) fn set_automation_min_value(
        &mut self,
        track_id: &TrackID,
        node_id: &NodeID,
        min_value: KeyframeValue,
    ) {
        if let Some(node) = self.get_automation_node_mut(track_id, node_id) {
            match &mut node.track {
                AutomationTrack::Float { range, .. } => {
                    if let KeyframeValue::Float(min_value) = min_value {
                        *range = min_value.min(*range.end())..=*range.end();
                        self.modified_project();
                    }
                }
                AutomationTrack::Int { range, .. } => {
                    if let KeyframeValue::Int(min_value) = min_value {
                        *range = min_value.min(*range.end())..=*range.end();
                        self.modified_project();
                    }
                }
                AutomationTrack::Bool { .. } => (),
            }
        }
    }
}
