use eframe::egui;
use kadent_engine::graph::node_id::NodeID;

pub struct NodeGraphState {
    /// Currently being dragged edge, with the source and the mouse position.
    pub ghost_edge: Option<((NodeID, usize), egui::Pos2)>,
    /// The node that should disappear when dragging an edge, to avoid visual confusion.
    pub dragged_edge: Option<(NodeID, usize, NodeID, usize)>,
    /// User pan, relative to the content area top-left.
    /// Combined with content_rect.min each frame, so the view follows panel moves/resizes.
    pub pan_offset: egui::Vec2,
    /// If set, pan will be updated this frame to center on this canvas-space position.
    pub jump_to_pos: Option<egui::Pos2>,
}

impl Default for NodeGraphState {
    fn default() -> Self {
        Self {
            ghost_edge: None,
            dragged_edge: None,
            // Start with a small margin so canvas (0, 0) is visible by default
            pan_offset: egui::vec2(50.0, 50.0),
            jump_to_pos: None,
        }
    }
}
