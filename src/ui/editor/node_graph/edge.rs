use eframe::egui;
use knodiq_engine::{graph::node_id::NodeID, mixer::TrackID};

use super::node::{canvas_to_screen, input_port_pos, output_port_pos};
use crate::{colors, ui::EditorUi};

impl EditorUi {
    pub(super) fn draw_graph_edge(
        &self,
        painter: &egui::Painter,
        track_id: TrackID,
        edge: &(NodeID, usize, NodeID, usize),
        pan: egui::Vec2,
        origin: egui::Pos2,
    ) {
        // If the edge is being dragged, skip drawing it here to avoid visual confusion
        if self
            .ui_state
            .node_graph_state
            .dragged_edge
            .is_some_and(|ref dragged_edge| dragged_edge == edge)
        {
            return;
        }

        let (from_id, out_idx, to_id, in_idx) = *edge;

        let Some(track_meta) = self.project_meta.get_track(&track_id) else {
            return;
        };
        let Some(from_canvas) = track_meta.node_graph.get_node_pos(from_id) else {
            return;
        };
        let Some(to_canvas) = track_meta.node_graph.get_node_pos(to_id) else {
            return;
        };

        let from_pos = output_port_pos(canvas_to_screen(from_canvas, pan, origin), out_idx);
        let to_pos = input_port_pos(canvas_to_screen(to_canvas, pan, origin), in_idx);

        draw_bezier_edge(painter, from_pos, to_pos);
    }

    pub(super) fn draw_dragged_edge(
        &self,
        painter: &egui::Painter,
        track_id: TrackID,
        from: (NodeID, usize),
        to: egui::Pos2,
        pan: egui::Vec2,
        origin: egui::Pos2,
    ) {
        let Some(track_meta) = self.project_meta.get_track(&track_id) else {
            return;
        };
        let Some(from_canvas) = track_meta.node_graph.get_node_pos(from.0) else {
            return;
        };

        let from_pos = output_port_pos(canvas_to_screen(from_canvas, pan, origin), from.1);

        draw_bezier_edge(painter, from_pos, to);
    }
}

fn draw_bezier_edge(painter: &egui::Painter, from: egui::Pos2, to: egui::Pos2) {
    let cp_dist = ((to.x - from.x).abs() * 0.5).max(60.0);
    let cp1 = egui::pos2(from.x + cp_dist, from.y);
    let cp2 = egui::pos2(to.x - cp_dist, to.y);

    let stroke = egui::Stroke::new(2.0, colors::node_edge());

    let points: Vec<egui::Pos2> = (0..=24)
        .map(|i| cubic_bezier(from, cp1, cp2, to, i as f32 / 24.0))
        .collect();

    for pair in points.windows(2) {
        painter.line_segment([pair[0], pair[1]], stroke);
    }
}

fn cubic_bezier(
    p0: egui::Pos2,
    p1: egui::Pos2,
    p2: egui::Pos2,
    p3: egui::Pos2,
    t: f32,
) -> egui::Pos2 {
    let u = 1.0 - t;
    let uu = u * u;
    let uuu = uu * u;
    let tt = t * t;
    let ttt = tt * t;
    egui::pos2(
        uuu * p0.x + 3.0 * uu * t * p1.x + 3.0 * u * tt * p2.x + ttt * p3.x,
        uuu * p0.y + 3.0 * uu * t * p1.y + 3.0 * u * tt * p2.y + ttt * p3.y,
    )
}
