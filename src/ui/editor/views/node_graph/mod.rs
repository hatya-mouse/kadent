mod edge;
mod header;
mod node;
mod port;

use crate::core::audio_engine::graph::{InputKey, OutputKey};
use crate::core::audio_engine::{graph::node_id::NodeID, mixer::TrackID};
use crate::{ui::EditorState, ui::editor::actions::EditorAction};
use edge::{draw_edges, draw_ghost_edge};
use eframe::egui;
use port::find_hovered_input;

// --- NODE LAYOUT CONSTANTS ---
const NODE_WIDTH: f32 = 180.0;
const PORT_ROW_HEIGHT: f32 = 22.0;
const PORT_RADIUS: f32 = 8.0;
/// Padding on top and bottom of the node body.
const NODE_PADDING: f32 = 4.0;
/// Thinkness of the edge lines.
const EDGE_WIDTH: f32 = 4.0;
// Height of the node header, which is the same as the height of the node title bar.
const NODE_HEADER_HEGIHT: f32 = 24.0;

pub(crate) struct NodeGraphState {
    /// Currently being dragged edge, with the source and the mouse position.
    pub(crate) ghost_edge: Option<((NodeID, usize), egui::Pos2)>,
    /// The node that should disappear when dragging an edge, to avoid visual confusion.
    pub(crate) dragged_edge: Option<(NodeID, usize, NodeID, usize)>,
    /// User pan, relative to the content area top-left.
    /// Combined with content_rect.min each frame, so the view follows panel moves/resizes.
    pub(crate) pan_offset: egui::Vec2,
    /// If set, pan will be updated this frame to center on this canvas-space position.
    pub(crate) jump_to_pos: Option<egui::Pos2>,
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

impl NodeGraphState {
    pub(in crate::ui::editor) fn ui(&mut self, ui: &mut egui::Ui, state: &mut EditorState) {
        let content_rect = ui.available_rect_before_wrap();

        // Allocate the full canvas rect for background interaction.
        // Must be before node allocations so node interactions take priority.
        let bg_response = ui.allocate_rect(content_rect, egui::Sense::drag());

        // Middle mouse drag to pan
        if bg_response.dragged_by(egui::PointerButton::Middle) {
            self.pan_offset += bg_response.drag_delta();
        }

        // Center on a requested canvas position (e.g. "jump to random node")
        if let Some(target) = self.jump_to_pos.take() {
            let half_size = content_rect.size() * 0.5;
            self.pan_offset = egui::vec2(half_size.x - target.x, half_size.y - target.y);
        }

        // view_transform converts canvas-space positions to screen-space each frame.
        // Adding content_rect.min ensures nodes follow panel moves and resizes automatically.
        let view_transform = content_rect.min.to_vec2() + self.pan_offset;

        let Some(track_id) = state.selection.track_id() else {
            return;
        };

        // Collect what we need up front to avoid holding borrows during drawing
        let node_ids: Vec<NodeID> = state
            .project
            .meta
            .get_track(&track_id)
            .map(|t| t.graph.nodes.keys().cloned().collect())
            .unwrap_or_default();

        let edges = state
            .project
            .data
            .get_track(&track_id)
            .map(|t| t.get_graph().get_all_edges())
            .unwrap_or_default();

        // Draw edges behind nodes
        if let Some(track_meta) = state.project.meta.get_track(&track_id) {
            let painter = ui.painter();
            draw_edges(
                ui,
                painter,
                &edges,
                &track_meta.graph,
                view_transform,
                self.dragged_edge,
            );

            // Draw the ghost edge
            if let Some(ghost) = self.ghost_edge {
                draw_ghost_edge(ui, painter, &ghost, &track_meta.graph, view_transform);
            }
        }

        // Draw each node (on top of edges)
        for node_id in &node_ids {
            self.draw_node(ui, state, node_id, view_transform);
        }

        // Handle ghost edge release after drawing nodes
        if let Some(ghost) = self.ghost_edge {
            self.handle_ghost_release(ui, ghost, &track_id, &node_ids, view_transform, state);
        }
    }

    fn handle_ghost_release(
        &mut self,
        ui: &egui::Ui,
        ghost_edge: ((NodeID, usize), egui::Pos2),
        track_id: &TrackID,
        node_ids: &[NodeID],
        view_transform: egui::Vec2,
        state: &mut EditorState,
    ) {
        // Check if the ghost node has been released in this very frame
        if !ui.input(|i| i.pointer.primary_released()) {
            return;
        }

        if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos())
            && let Some(track_meta) = state.project.meta.get_track(track_id)
            && let Some(graph) = state
                .project
                .data
                .get_track(track_id)
                .map(|t| t.get_graph())
        {
            let mut has_connected_to_input = false;

            for node_id in node_ids {
                // Get the node metadata for its position
                let Some(node_meta) = track_meta.graph.get_node_meta(node_id) else {
                    continue;
                };

                // Get the number of ports from the project
                let Some(node) = graph.get_node(node_id) else {
                    continue;
                };

                // If the mouse is hovering over an input port, connect the dragged node to that port
                if let Some(port) = find_hovered_input(
                    mouse_pos,
                    node_meta.pos,
                    node.get_input_len(),
                    view_transform,
                ) {
                    // Remove the dragged node from the project and add a new edge to the hovered port
                    if let Some(old_edge) = self.dragged_edge {
                        state.actions.push_action(EditorAction::RemoveEdge(
                            *track_id,
                            InputKey(old_edge.2, old_edge.3),
                        ));
                    }

                    // Add the new edge to the project
                    let new_edge = (ghost_edge.0.0, ghost_edge.0.1, *node_id, port);
                    state.actions.push_action(EditorAction::AddEdge(
                        *track_id,
                        OutputKey(new_edge.0, new_edge.1),
                        InputKey(new_edge.2, new_edge.3),
                    ));

                    // Mark that we've connected the dragged edge to input
                    has_connected_to_input = true;

                    break;
                }
            }

            if !has_connected_to_input {
                // If we didn't connect to an input, just remove the dragged edge from the project
                // because it has released in empty space
                if let Some(old_edge) = self.dragged_edge {
                    state.actions.push_action(EditorAction::RemoveEdge(
                        *track_id,
                        InputKey(old_edge.2, old_edge.3),
                    ));
                }
            }
        }

        // Clear the ghost node
        self.ghost_edge = None;
        self.dragged_edge = None;
    }
}
