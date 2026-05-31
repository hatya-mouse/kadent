mod edge;
mod header;
mod node;
mod port;

// --- NODE LAYOUT CONSTANTS ---

pub(super) const NODE_WIDTH: f32 = 180.0;
pub(super) const HEADER_HEIGHT: f32 = 28.0;
pub(super) const PORT_ROW_HEIGHT: f32 = 22.0;
pub(super) const PORT_RADIUS: f32 = 8.0;
/// Padding on top and bottom of the node body.
pub(super) const NODE_PADDING: f32 = 4.0;
/// Thinkness of the edge lines.
pub(super) const EDGE_WIDTH: f32 = 4.0;

use crate::ui::{
    EditorUi,
    editor::node_graph::{
        edge::{draw_edges, draw_ghost_edge},
        port::find_hovered_input,
    },
};
use eframe::egui;
use kreniq_engine::{graph::node_id::NodeID, mixer::TrackID};

impl EditorUi {
    pub(in crate::ui) fn node_graph(&mut self, ui: &mut egui::Ui) {
        // Draw the node graph header
        self.draw_node_graph_header(ui);

        let content_rect = ui.available_rect_before_wrap();

        // Allocate the full canvas rect for background interaction.
        // Must be before node allocations so node interactions take priority.
        let bg_response = ui.allocate_rect(content_rect, egui::Sense::drag());

        // Middle mouse drag to pan
        if bg_response.dragged_by(egui::PointerButton::Middle) {
            self.ui_state.node_graph_state.pan_offset += bg_response.drag_delta();
        }

        // Center on a requested canvas position (e.g. "jump to random node")
        if let Some(target) = self.ui_state.node_graph_state.jump_to_pos.take() {
            let half_size = content_rect.size() / 2.0;
            self.ui_state.node_graph_state.pan_offset =
                egui::vec2(half_size.x - target.x, half_size.y - target.y);
        }

        // view_transform converts canvas-space positions to screen-space each frame.
        // Adding content_rect.min ensures nodes follow panel moves and resizes automatically.
        let view_transform = content_rect.min.to_vec2() + self.ui_state.node_graph_state.pan_offset;

        let Some(track_id) = self.ui_state.selected_track else {
            return;
        };

        // Collect what we need up front to avoid holding borrows during drawing
        let node_ids: Vec<NodeID> = self
            .project_meta
            .get_track(&track_id)
            .map(|t| t.graph.nodes.keys().cloned().collect())
            .unwrap_or_default();

        let edges = self
            .project
            .get_track(&track_id)
            .map(|t| t.get_graph().get_edges().clone())
            .unwrap_or_default();

        // Copy ghost/dragged edge before borrowing project_meta below
        let ghost_edge = self.ui_state.node_graph_state.ghost_edge;
        let dragged_edge = self.ui_state.node_graph_state.dragged_edge;

        // Draw edges behind nodes
        if let Some(track_meta) = self.project_meta.get_track(&track_id) {
            let painter = ui.painter();
            draw_edges(
                ui,
                painter,
                &edges,
                &track_meta.graph,
                view_transform,
                dragged_edge,
            );

            // Draw the ghost edge
            if let Some(ghost) = ghost_edge {
                draw_ghost_edge(ui, painter, &ghost, &track_meta.graph, view_transform);
            }
        }

        // Draw each node (on top of edges)
        for node_id in &node_ids {
            self.draw_node(ui, node_id, view_transform);
        }

        // Handle ghost edge release after drawing nodes

        if let Some(ghost) = ghost_edge {
            self.handle_ghost_release(ui, ghost, &track_id, &node_ids, view_transform);
        }
    }

    fn handle_ghost_release(
        &mut self,
        ui: &egui::Ui,
        ghost_edge: ((NodeID, usize), egui::Pos2),
        track_id: &TrackID,
        node_ids: &[NodeID],
        view_transform: egui::Vec2,
    ) {
        // Check if the ghost node has been released in this very frame
        if !ui.input(|i| i.pointer.primary_released()) {
            return;
        }

        if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos())
            && let Some(track_meta) = self.project_meta.get_track(track_id)
            && let Some(graph) = self.project.get_track(track_id).map(|t| t.get_graph())
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
                    if let Some(old_edge) = self.ui_state.node_graph_state.dragged_edge {
                        self.remove_edge(track_id, old_edge);
                    }

                    // Add the new edge to the project
                    let new_edge = (ghost_edge.0.0, ghost_edge.0.1, *node_id, port);
                    self.add_edge(track_id, new_edge);

                    // Mark that we've connected the dragged edge to input
                    has_connected_to_input = true;

                    break;
                }
            }

            if !has_connected_to_input {
                // If we didn't connect to an input, just remove the dragged edge from the project
                // because it has released in empty space
                if let Some(old_edge) = self.ui_state.node_graph_state.dragged_edge {
                    self.remove_edge(track_id, old_edge);
                }
            }
        }

        // Clear the ghost node
        self.ui_state.node_graph_state.ghost_edge = None;
        self.ui_state.node_graph_state.dragged_edge = None;
    }
}
