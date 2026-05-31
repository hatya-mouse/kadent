use crate::{
    kasl_node::KaslNode,
    metadata::{NodeMeta, NodeType},
    ui::EditorUi,
};
use eframe::egui;
use kreniq_engine::mixer::TrackID;

impl EditorUi {
    pub(crate) fn add_kasl_node(&mut self, track_id: &TrackID, pos: egui::Pos2) {
        let mut kasl_node = KaslNode::new();
        kasl_node.set_search_paths(self.project_meta.kasl_search_paths.clone());
        kasl_node.set_project_dir(self.project_dir.clone());

        // Add the node to the project
        let Some(node_id) = self
            .project
            .get_track_mut(track_id)
            .map(|t| t.get_graph_mut().add_node(Box::new(kasl_node)))
        else {
            return;
        };

        // Also add the node to the project meta with the given position
        if let Some(track_meta) = self.project_meta.get_track_mut(track_id) {
            track_meta.graph.set_node_meta(
                node_id,
                NodeMeta::new(NodeType::Kasl, "KASL Node".to_string(), pos),
            );
        }

        self.modified_project();
    }
}
