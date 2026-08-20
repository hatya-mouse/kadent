use crate::ui::{
    EditorState,
    components::text_input::text_input_with_callback,
    editor::{
        actions::{EditorAction, KeyframeValue},
        views::inspector::{inspector_item, inspector_section},
    },
    theme,
};
use eframe::egui;
use kadent_engine::{
    graph::node_id::NodeID,
    mixer::TrackID,
    node::builtin::{AutomationNode, AutomationTrack},
};

impl EditorState {
    pub(super) fn keyframe_inspector(
        &mut self,
        ui: &mut egui::Ui,
        track_id: &TrackID,
        node_id: &NodeID,
        keyframe_index: usize,
    ) {
        inspector_section(
            ui,
            ("keyframe_section", track_id, node_id, keyframe_index),
            "Keyframe",
            |ui| {
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

                match track {
                    AutomationTrack::Float { keyframes, .. } => {
                        let Some(keyframe) = keyframes.get_mut(keyframe_index) else {
                            return;
                        };
                        inspector_item(ui, "Value", |ui| {
                            text_input_with_callback(ui, keyframe.value.to_string(), |new_value| {
                                if let Ok(new_value) = new_value.parse::<f32>() {
                                    self.actions.push_action(EditorAction::SetKeyframeValue(
                                        *track_id,
                                        *node_id,
                                        keyframe_index,
                                        KeyframeValue::Float(new_value),
                                    ));
                                }
                            });
                        });
                    }
                    AutomationTrack::Int { keyframes, .. } => {
                        let Some(keyframe) = keyframes.get_mut(keyframe_index) else {
                            return;
                        };
                        inspector_item(ui, "Value", |ui| {
                            text_input_with_callback(ui, keyframe.value.to_string(), |new_value| {
                                if let Ok(new_value) = new_value.parse::<i32>() {
                                    self.actions.push_action(EditorAction::SetKeyframeValue(
                                        *track_id,
                                        *node_id,
                                        keyframe_index,
                                        KeyframeValue::Int(new_value),
                                    ));
                                }
                            });
                        });
                    }
                    AutomationTrack::Bool { keyframes, .. } => {
                        let Some(keyframe) = keyframes.get_mut(keyframe_index) else {
                            return;
                        };
                        inspector_item(ui, "Value", |ui| {
                            let response = ui.checkbox(&mut keyframe.value, "Value");
                            if response.changed() {
                                self.actions.push_action(EditorAction::SetKeyframeValue(
                                    *track_id,
                                    *node_id,
                                    keyframe_index,
                                    KeyframeValue::Bool(!keyframe.value),
                                ));
                            }
                        });
                    }
                }

                if self.debug_mode {
                    inspector_item(ui, "Keyframe Index", |ui| {
                        ui.label(
                            egui::RichText::new(format!("{}", keyframe_index))
                                .size(theme::normal_font_size()),
                        );
                    });
                }
            },
        );
    }
}
