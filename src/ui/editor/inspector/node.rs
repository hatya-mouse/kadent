use crate::{
    components::text_input::{text_input, text_input_with_callback},
    kasl_node::KaslNode,
    theme,
    ui::{
        EditorUi,
        editor::inspector::{inspector_item, inspector_section},
    },
};
use eframe::egui;
use kreniq_engine::{graph::node_id::NodeID, mixer::TrackID};

impl EditorUi {
    pub(super) fn node_inspector(
        &mut self,
        ui: &mut egui::Ui,
        track_id: &TrackID,
        node_id: &NodeID,
    ) {
        inspector_section(ui, ("node_section", track_id, node_id), "Node", |ui| {
            let Some(track_meta) = self.project_meta.get_track_mut(track_id) else {
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
                inspector_item(ui, "Track ID", |ui| {
                    ui.label(
                        egui::RichText::new(format!("{}", track_id.0))
                            .size(theme::normal_font_size()),
                    );
                });
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
        let Some(track) = self.project.get_track_mut(track_id) else {
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
                if ui.button("Compile KASL").clicked() {
                    self.compile_kasl_node(track_id, node_id);
                }
            });
        }
    }
}
