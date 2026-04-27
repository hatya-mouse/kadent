use crate::{
    components::text_input::text_input,
    theme,
    ui::{
        EditorUi,
        editor::inspector::{inspector_item, inspector_section},
    },
};
use eframe::egui;
use knodiq_engine::{graph::node_id::NodeID, mixer::TrackID};

impl EditorUi {
    pub(super) fn node_inspector(
        &mut self,
        ui: &mut egui::Ui,
        track_id: &TrackID,
        node_id: &NodeID,
    ) {
        let Some(track_meta) = self.project_meta.get_track_mut(track_id) else {
            return;
        };
        let Some(node_meta) = track_meta.graph.get_node_meta_mut(node_id) else {
            return;
        };

        inspector_section(ui, "Node".to_string(), |ui| {
            inspector_item(ui, "Name", |ui| {
                text_input(ui, &mut node_meta.display_name);
            });

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
}
