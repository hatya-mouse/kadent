use eframe::egui;
use knodiq_engine::graph::node_id::NodeID;

#[derive(Default)]
pub struct NodeGraphState {
    /// Currently being dragged edge, with the source and the mouse position.
    pub ghost_edge: Option<((NodeID, usize), egui::Pos2)>,
    /// The node that should disappear when dragging an edge, to avoid visual confusion.
    pub dragged_edge: Option<(NodeID, usize, NodeID, usize)>,
    /// Canvas-space position where the node graph context menu was opened.
    pub node_graph_add_pos: Option<egui::Pos2>,
    /// Currently selected node ID.
    pub selected_node: Option<NodeID>,
}
