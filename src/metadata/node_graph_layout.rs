use eframe::egui;
use knodiq_engine::graph::node_id::NodeID;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub(crate) struct NodeGraphLayout {
    /// The offset of the node graph canvas, in screen space. This is used for panning the view.
    pub pan_offset: egui::Vec2,
    /// The positions of nodes in canvas space.
    pub node_positions: HashMap<NodeID, egui::Pos2>,
}

impl NodeGraphLayout {
    pub fn get_node_pos(&self, node_id: NodeID) -> Option<egui::Pos2> {
        self.node_positions.get(&node_id).copied()
    }

    pub fn set_node_pos(&mut self, node_id: NodeID, pos: egui::Pos2) {
        self.node_positions.insert(node_id, pos);
    }

    pub fn ensure_node_pos(&mut self, node_id: NodeID, default: egui::Pos2) -> egui::Pos2 {
        *self.node_positions.entry(node_id).or_insert(default)
    }
}
