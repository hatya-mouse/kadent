use crate::ui::{
    EditorState,
    components::{dropdown::dropdown_button, text_input::text_input_with_callback},
    editor::{
        actions::{EditorAction, KeyframeValue},
        views::inspector::{inspector_item, inspector_section},
    },
    theme,
};
use crate::{
    core::audio_engine::{
        graph::node_id::NodeID,
        mixer::TrackID,
        node::builtin::{AutomationNode, AutomationTrack, CurveType},
    },
    ui::editor::views::inspector::InspectorView,
};
use eframe::egui;

impl InspectorView {
    pub(super) fn keyframe_inspector(
        &mut self,
        ui: &mut egui::Ui,
        editor_state: &mut EditorState,
        track_id: &TrackID,
        node_id: &NodeID,
        keyframe_index: usize,
    ) {
        inspector_section(
            ui,
            ("keyframe_section", track_id, node_id, keyframe_index),
            "Keyframe",
            |ui| {
                self.keyframe_inspector_item(ui, editor_state, track_id, node_id, keyframe_index);
            },
        );
    }

    fn keyframe_inspector_item(
        &mut self,
        ui: &mut egui::Ui,
        editor_state: &mut EditorState,
        track_id: &TrackID,
        node_id: &NodeID,
        keyframe_index: usize,
    ) {
        let Some(track) = editor_state
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
                            editor_state
                                .actions
                                .push_action(EditorAction::SetKeyframeValue(
                                    *track_id,
                                    *node_id,
                                    keyframe_index,
                                    KeyframeValue::Float(new_value),
                                ));
                        }
                    });
                });

                inspector_item(ui, "Curve Type", |ui| {
                    dropdown_button(
                        ui,
                        ui.id().with("curve_type"),
                        curve_type_to_string(&keyframe.curve),
                        |ui| {
                            for curve_type in CurveType::all() {
                                if ui
                                    .selectable_label(
                                        keyframe.curve.is_same_type(curve_type),
                                        curve_type_to_string(curve_type),
                                    )
                                    .clicked()
                                {
                                    editor_state.actions.push_action(
                                        EditorAction::SetKeyframeCurveType(
                                            *track_id,
                                            *node_id,
                                            keyframe_index,
                                            *curve_type,
                                        ),
                                    );
                                }
                            }
                        },
                    );
                });
            }
            AutomationTrack::Int { keyframes, .. } => {
                let Some(keyframe) = keyframes.get_mut(keyframe_index) else {
                    return;
                };
                inspector_item(ui, "Value", |ui| {
                    text_input_with_callback(ui, keyframe.value.to_string(), |new_value| {
                        if let Ok(new_value) = new_value.parse::<i32>() {
                            editor_state
                                .actions
                                .push_action(EditorAction::SetKeyframeValue(
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
                        editor_state
                            .actions
                            .push_action(EditorAction::SetKeyframeValue(
                                *track_id,
                                *node_id,
                                keyframe_index,
                                KeyframeValue::Bool(!keyframe.value),
                            ));
                    }
                });
            }
        }

        if editor_state.debug_mode {
            inspector_item(ui, "Keyframe Index", |ui| {
                ui.label(
                    egui::RichText::new(format!("{}", keyframe_index))
                        .size(theme::normal_font_size()),
                );
            });
        }
    }
}

fn curve_type_to_string(curve_type: &CurveType) -> &'static str {
    match curve_type {
        CurveType::Linear => "Linear",
        CurveType::Step => "Step",
        CurveType::Smooth { tension: _ } => "Smooth",
    }
}
