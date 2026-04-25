use crate::{colors, ui::EditorUi};
use eframe::egui;
use knodiq_engine::mixer::TrackID;

use super::NodeDrawData;

pub const NODE_WIDTH: f32 = 140.0;
pub const NODE_HEADER_HEIGHT: f32 = 24.0;
pub const PORT_ROW_HEIGHT: f32 = 18.0;
pub const PORT_RADIUS: f32 = 5.0;

pub fn node_height(input_count: usize, output_count: usize) -> f32 {
    NODE_HEADER_HEIGHT + PORT_ROW_HEIGHT * input_count.max(output_count).max(1) as f32
}

pub fn canvas_to_screen(canvas_pos: egui::Pos2, pan: egui::Vec2, origin: egui::Pos2) -> egui::Pos2 {
    origin + canvas_pos.to_vec2() + pan
}

pub fn input_port_pos(node_top_left: egui::Pos2, index: usize) -> egui::Pos2 {
    egui::pos2(
        node_top_left.x,
        node_top_left.y
            + NODE_HEADER_HEIGHT
            + PORT_ROW_HEIGHT * index as f32
            + PORT_ROW_HEIGHT * 0.5,
    )
}

pub fn output_port_pos(node_top_left: egui::Pos2, index: usize) -> egui::Pos2 {
    egui::pos2(
        node_top_left.x + NODE_WIDTH,
        node_top_left.y
            + NODE_HEADER_HEIGHT
            + PORT_ROW_HEIGHT * index as f32
            + PORT_ROW_HEIGHT * 0.5,
    )
}

impl EditorUi {
    /// Draws the node and handles drag interaction. Returns true if the node was dragged this frame.
    pub(super) fn draw_and_interact_node(
        &mut self,
        ui: &mut egui::Ui,
        painter: &egui::Painter,
        track_id: TrackID,
        draw_data: &NodeDrawData,
        canvas_pos: egui::Pos2,
        pan: egui::Vec2,
        origin: egui::Pos2,
    ) -> bool {
        let screen_pos = canvas_to_screen(canvas_pos, pan, origin);
        let h = node_height(draw_data.input_names.len(), draw_data.output_names.len());
        let node_rect = egui::Rect::from_min_size(screen_pos, egui::vec2(NODE_WIDTH, h));

        let drag_id = ui.id().with("node_drag").with(draw_data.id);
        let drag_resp = ui.interact(node_rect, drag_id, egui::Sense::drag());

        if drag_resp.dragged() && self.ui_state.node_graph_state.ghost_edge.is_none() {
            let new_pos = canvas_pos + drag_resp.drag_delta();
            if let Some(track_meta) = self.project_meta.get_track_mut(&track_id) {
                track_meta.node_graph.set_node_pos(draw_data.id, new_pos);
            }
        }

        let dark_mode = ui.visuals().dark_mode;
        draw_node_body(
            painter,
            node_rect,
            &draw_data.label,
            &draw_data.input_names,
            &draw_data.output_names,
            dark_mode,
        );

        drag_resp.dragged()
    }
}

fn draw_node_body(
    painter: &egui::Painter,
    node_rect: egui::Rect,
    label: &str,
    input_names: &[String],
    output_names: &[String],
    dark_mode: bool,
) {
    painter.rect(
        node_rect,
        4.0,
        colors::tertiary_bg(dark_mode),
        egui::Stroke::new(1.0, colors::border(dark_mode)),
        egui::StrokeKind::Inside,
    );

    let separator_y = node_rect.min.y + NODE_HEADER_HEIGHT;
    painter.hline(
        node_rect.min.x..=node_rect.max.x,
        separator_y,
        egui::Stroke::new(1.0, colors::border(dark_mode)),
    );

    painter.text(
        egui::pos2(
            node_rect.center().x,
            node_rect.min.y + NODE_HEADER_HEIGHT * 0.5,
        ),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(12.0),
        colors::primary_fg(dark_mode),
    );

    draw_ports(painter, node_rect.min, input_names, output_names, dark_mode);
}

fn draw_ports(
    painter: &egui::Painter,
    node_top_left: egui::Pos2,
    input_names: &[String],
    output_names: &[String],
    dark_mode: bool,
) {
    let fg = colors::primary_fg(dark_mode);
    let input_color = colors::node_port_input();
    let output_color = colors::node_port_output();

    for (i, name) in input_names.iter().enumerate() {
        let pos = input_port_pos(node_top_left, i);
        painter.circle_filled(pos, PORT_RADIUS, input_color);
        painter.text(
            egui::pos2(pos.x + PORT_RADIUS + 4.0, pos.y),
            egui::Align2::LEFT_CENTER,
            name,
            egui::FontId::proportional(10.0),
            fg,
        );
    }

    for (i, name) in output_names.iter().enumerate() {
        let pos = output_port_pos(node_top_left, i);
        painter.circle_filled(pos, PORT_RADIUS, output_color);
        painter.text(
            egui::pos2(pos.x - PORT_RADIUS - 4.0, pos.y),
            egui::Align2::RIGHT_CENTER,
            name,
            egui::FontId::proportional(10.0),
            fg,
        );
    }
}
