use crate::{
    actions::AddibleNodes,
    core::{
        kasl_node::KaslNode,
        metadata::{NodeMeta, NodeType},
    },
    storage::project::get_project_dir,
    ui::EditorState,
};
use eframe::egui;
use kadent_engine::{
    mixer::TrackID,
    node::builtin::{AutomationNode, AutomationTrack},
};

impl EditorState {
    pub(in crate::actions) fn add_node(
        &mut self,
        track_id: &TrackID,
        node_type: &AddibleNodes,
        pos: egui::Pos2,
    ) {
        match node_type {
            AddibleNodes::Kasl => self.add_kasl_node(track_id, pos),
            AddibleNodes::Automation => self.add_automation_node(track_id, pos),
        }
    }

    fn add_kasl_node(&mut self, track_id: &TrackID, pos: egui::Pos2) {
        let mut kasl_node = KaslNode::new();
        let project_dir = get_project_dir(&self.ui_state.proj_ctx.project_path);
        kasl_node.set_search_paths(
            self.ui_state
                .proj_ctx
                .project_meta
                .kasl_search_paths
                .clone(),
        );
        kasl_node.set_project_dir(project_dir);

        // Add the node to the project
        let Some(node_id) = self
            .ui_state
            .proj_ctx
            .project
            .get_track_mut(track_id)
            .map(|t| t.get_graph_mut().add_node(Box::new(kasl_node)))
        else {
            return;
        };

        // Also add the node to the project meta with the given position
        if let Some(track_meta) = self.ui_state.proj_ctx.project_meta.get_track_mut(track_id) {
            track_meta.graph.set_node_meta(
                node_id,
                NodeMeta::new(NodeType::Kasl, "KASL Node".to_string(), pos),
            );
        }

        self.modified_project();
    }

    fn add_automation_node(&mut self, track_id: &TrackID, pos: egui::Pos2) {
        // Add the node to the project
        let track = AutomationTrack::new_float();
        let Some(node_id) = self
            .ui_state
            .proj_ctx
            .project
            .get_track_mut(track_id)
            .map(|t| {
                t.get_graph_mut()
                    .add_node(Box::new(AutomationNode::new(track)))
            })
        else {
            return;
        };

        // Also add the node to the project meta with the given position
        if let Some(track_meta) = self.ui_state.proj_ctx.project_meta.get_track_mut(track_id) {
            track_meta.graph.set_node_meta(
                node_id,
                NodeMeta::new(NodeType::Automation, "Automation Node".to_string(), pos),
            );
        }

        self.modified_project();
    }
}
