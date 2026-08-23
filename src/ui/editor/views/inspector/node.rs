use super::{inspector_item, inspector_section};
use crate::core::audio_engine::{
    graph::node_id::NodeID,
    mixer::TrackID,
    node::builtin::{AutomationNode, AutomationTrack, AutomationTrackType},
};
use crate::{
    core::kasl_node::KaslNode,
    ui::{
        EditorState,
        components::{
            dropdown::dropdown_button,
            text_button::text_button,
            text_input::{text_input, text_input_with_callback},
        },
        editor::actions::{EditorAction, KeyframeValue},
        theme,
    },
};
use eframe::egui;

impl EditorState {
    pub(super) fn node_inspector(
        &mut self,
        ui: &mut egui::Ui,
        track_id: &TrackID,
        node_id: &NodeID,
    ) {
        inspector_section(ui, ("node_section", track_id, node_id), "Node", |ui| {
            let Some(track_meta) = self.project.meta.get_track_mut(track_id) else {
                return;
            };
            let Some(node_meta) = track_meta.graph.get_node_meta_mut(node_id) else {
                return;
            };

            inspector_item(ui, "Name", |ui| {
                text_input(ui, &mut node_meta.display_name);
            });

            self.node_unique_inspector(ui, track_id, node_id);

            if self.debug_mode {
                ui.separator();
                inspector_item(ui, "Node ID", |ui| {
                    ui.label(
                        egui::RichText::new(format!("{}", node_id.0))
                            .size(theme::normal_font_size()),
                    );
                });
            }
        });
    }

    fn node_unique_inspector(&mut self, ui: &mut egui::Ui, track_id: &TrackID, node_id: &NodeID) {
        let Some(track) = self.project.data.get_track_mut(track_id) else {
            return;
        };
        let Some(node) = track.get_graph_mut().get_node_mut(node_id) else {
            return;
        };

        if let Some(kasl_node) = node.as_any_mut().downcast_mut::<KaslNode>() {
            inspector_item(ui, "KASL Path", |ui| {
                text_input_with_callback(
                    ui,
                    kasl_node.get_file_path().cloned().unwrap_or_default(),
                    |new_path| {
                        kasl_node.set_file_path(new_path.clone());
                    },
                );
            });

            inspector_item(ui, "Compile", |ui| {
                if text_button(ui, "compile_kasl", "Compile KASL").clicked() {
                    self.actions
                        .push_action(EditorAction::CompileKasl(*track_id, *node_id));
                }
            });
        } else if let Some(automation_node) = node.as_any_mut().downcast_mut::<AutomationNode>() {
            let current_track_type = automation_node.track.track_type();

            inspector_item(ui, "Track Type", |ui| {
                dropdown_button(
                    ui,
                    ui.id().with("track_type"),
                    track_type_to_string(&current_track_type),
                    |ui| {
                        for track_type in AutomationTrackType::all() {
                            if ui
                                .selectable_label(
                                    current_track_type == *track_type,
                                    track_type_to_string(track_type),
                                )
                                .clicked()
                            {
                                self.actions.push_action(EditorAction::SetAutomationType(
                                    *track_id,
                                    *node_id,
                                    *track_type,
                                ));
                            }
                        }
                    },
                );
            });

            match &automation_node.track {
                AutomationTrack::Float { range, .. } => {
                    inspector_item(ui, "Max", |ui| {
                        text_input_with_callback(ui, range.end().to_string(), |new_max| {
                            if let Ok(new_max) = new_max.parse::<f32>() {
                                self.actions
                                    .push_action(EditorAction::SetAutomationMaxValue(
                                        *track_id,
                                        *node_id,
                                        KeyframeValue::Float(new_max),
                                    ));
                            }
                        });
                    });
                    inspector_item(ui, "Min", |ui| {
                        text_input_with_callback(ui, range.start().to_string(), |new_min| {
                            if let Ok(new_min) = new_min.parse::<f32>() {
                                self.actions
                                    .push_action(EditorAction::SetAutomationMinValue(
                                        *track_id,
                                        *node_id,
                                        KeyframeValue::Float(new_min),
                                    ));
                            }
                        });
                    });
                }
                AutomationTrack::Int { range, .. } => {
                    inspector_item(ui, "Max", |ui| {
                        text_input_with_callback(ui, range.end().to_string(), |new_max| {
                            if let Ok(new_max) = new_max.parse::<i32>() {
                                self.actions
                                    .push_action(EditorAction::SetAutomationMaxValue(
                                        *track_id,
                                        *node_id,
                                        KeyframeValue::Int(new_max),
                                    ));
                            }
                        });
                    });
                    inspector_item(ui, "Min", |ui| {
                        text_input_with_callback(ui, range.start().to_string(), |new_min| {
                            if let Ok(new_min) = new_min.parse::<i32>() {
                                self.actions
                                    .push_action(EditorAction::SetAutomationMinValue(
                                        *track_id,
                                        *node_id,
                                        KeyframeValue::Int(new_min),
                                    ));
                            }
                        });
                    });
                }
                AutomationTrack::Bool { .. } => (),
            }
        }
    }
}

fn track_type_to_string(track_type: &AutomationTrackType) -> &'static str {
    match track_type {
        AutomationTrackType::Float => "Float",
        AutomationTrackType::Int => "Int",
        AutomationTrackType::Bool => "Bool",
    }
}
