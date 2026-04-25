use crate::ui::EditorUi;
use eframe::egui;
use knodiq_engine::{graph::node_id::NodeID, mixer::TrackID};

use super::{
    NodeDrawData,
    node::{PORT_RADIUS, canvas_to_screen, input_port_pos},
};

impl EditorUi {
    /// Checks if the cursor is over a connected input port and starts a ghost edge drag.
    pub(super) fn try_start_edge_drag(
        &mut self,
        draw_data: &[NodeDrawData],
        edges: &[(NodeID, usize, NodeID, usize)],
        track_id: TrackID,
        pan: egui::Vec2,
        origin: egui::Pos2,
        cursor: egui::Pos2,
    ) {
        'hit: for data in draw_data {
            let Some(canvas_pos) = self
                .project_meta
                .get_track(&track_id)
                .and_then(|m| m.node_graph.get_node_pos(data.id))
            else {
                continue;
            };
            let screen_top_left = canvas_to_screen(canvas_pos, pan, origin);
            for in_index in 0..data.input_names.len() {
                let port_pos = input_port_pos(screen_top_left, in_index);
                if cursor.distance(port_pos) > PORT_RADIUS + 4.0 {
                    continue;
                }
                let Some(edge) = edges
                    .iter()
                    .find(|(_, _, to_id, to_index)| *to_id == data.id && *to_index == in_index)
                else {
                    continue;
                };
                self.ui_state.node_graph_state.ghost_edge = Some(((edge.0, edge.1), cursor));
                self.ui_state.node_graph_state.dragged_edge =
                    Some((edge.0, edge.1, data.id, in_index));
                break 'hit;
            }
        }
    }

    /// Updates the ghost edge cursor position each frame, or clears the drag on mouse release.
    pub(super) fn update_edge_drag(
        &mut self,
        cursor_pos: Option<egui::Pos2>,
        primary_released: bool,
    ) {
        if self.ui_state.node_graph_state.ghost_edge.is_none() {
            return;
        }
        if primary_released {
            self.ui_state.node_graph_state.ghost_edge = None;
            self.ui_state.node_graph_state.dragged_edge = None;
            return;
        }
        if let Some(cursor) = cursor_pos
            && let Some(ghost) = self.ui_state.node_graph_state.ghost_edge.as_mut()
        {
            ghost.1 = cursor;
        }
    }
}
