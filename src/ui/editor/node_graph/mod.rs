mod node;

use crate::ui::EditorUi;
use eframe::egui;
use knodiq_engine::graph::node_id::NodeID;

impl EditorUi {
    pub(in crate::ui) fn node_graph(&mut self, ui: &mut egui::Ui) {
        // Get the node meta too
        let Some(node_ids): Option<Vec<NodeID>> = self
            .ui_state
            .selected_track
            .and_then(|track_id| self.project_meta.get_track(&track_id))
            .map(|track| track.graph.nodes.keys().cloned().collect())
        else {
            return;
        };

        // Draw the nodes
        for node_id in node_ids {
            // Draw the node
            self.draw_node(ui, &node_id);
        }
    }
}
