use eframe::egui;
use knodiq_engine::graph::node_id::NodeID;

#[derive(Default)]
pub struct NodeGraphState {
    /// Currently being dragged edge, with the source and the mouse position.
    pub ghost_edge: Option<((NodeID, usize), egui::Pos2)>,
    /// The node that should disappear when dragging an edge, to avoid visual confusion.
    pub dragged_edge: Option<(NodeID, usize, NodeID, usize)>,
}
