use super::{HEADER_HEIGHT, NODE_WIDTH, PORT_ROW_HEIGHT};
use crate::{
    metadata::GraphMeta,
    theme,
    ui::editor::node_graph::{EDGE_WIDTH, NODE_PADDING},
};
use eframe::egui;
use kreniq_engine::graph::node_id::NodeID;

/// Draws all edges in the graph as straight lines, skipping `dragged_edge` if set.
pub(super) fn draw_edges(
    ui: &egui::Ui,
    painter: &egui::Painter,
    edges: &[(NodeID, usize, NodeID, usize)],
    graph_meta: &GraphMeta,
    view_transform: egui::Vec2,
    dragged_edge: Option<(NodeID, usize, NodeID, usize)>,
) {
    let stroke = egui::Stroke::new(EDGE_WIDTH, theme::node_edge(ui.visuals().dark_mode));

    for &(from_id, out_idx, to_id, in_idx) in edges {
        if dragged_edge == Some((from_id, out_idx, to_id, in_idx)) {
            continue;
        }
        let Some(from_meta) = graph_meta.get_node_meta(&from_id) else {
            continue;
        };
        let Some(to_meta) = graph_meta.get_node_meta(&to_id) else {
            continue;
        };

        let src = output_port_pos(from_meta.pos, out_idx, view_transform);
        let dst = input_port_pos(to_meta.pos, in_idx, view_transform);

        painter.line_segment([src, dst], stroke);
    }
}

/// Draws the ghost edge from an output port to the mouse cursor while dragging.
pub(super) fn draw_ghost_edge(
    ui: &egui::Ui,
    painter: &egui::Painter,
    ghost_edge: &((NodeID, usize), egui::Pos2),
    graph_meta: &GraphMeta,
    view_transform: egui::Vec2,
) {
    let ((src_id, out_idx), mouse_pos) = ghost_edge;
    let Some(src_meta) = graph_meta.get_node_meta(src_id) else {
        return;
    };

    let src = output_port_pos(src_meta.pos, *out_idx, view_transform);
    painter.line_segment(
        [src, *mouse_pos],
        egui::Stroke::new(EDGE_WIDTH, theme::node_edge(ui.visuals().dark_mode)),
    );
}

/// Screen-space position of an output port (right side of the node).
fn output_port_pos(
    canvas_pos: egui::Pos2,
    port_idx: usize,
    view_transform: egui::Vec2,
) -> egui::Pos2 {
    let screen_min = canvas_pos + view_transform;
    egui::pos2(
        screen_min.x + NODE_WIDTH,
        screen_min.y
            + HEADER_HEIGHT
            + NODE_PADDING
            + PORT_ROW_HEIGHT * port_idx as f32
            + PORT_ROW_HEIGHT / 2.0,
    )
}

/// Screen-space position of an input port (left side of the node).
fn input_port_pos(
    canvas_pos: egui::Pos2,
    port_idx: usize,
    view_transform: egui::Vec2,
) -> egui::Pos2 {
    let screen_min = canvas_pos + view_transform;
    egui::pos2(
        screen_min.x,
        screen_min.y
            + HEADER_HEIGHT
            + NODE_PADDING
            + PORT_ROW_HEIGHT * port_idx as f32
            + PORT_ROW_HEIGHT / 2.0,
    )
}
