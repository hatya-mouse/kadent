use super::{HEADER_HEIGHT, NODE_PADDING, NODE_WIDTH, PORT_RADIUS, PORT_ROW_HEIGHT};
use crate::{
    theme,
    ui::{
        EditorUi,
        editor::node_graph::port::{calc_port_y, draw_ports},
    },
};
use eframe::egui::{self, Sense};
use kreniq_engine::{graph::node_id::NodeID, mixer::TrackID};

impl EditorUi {
    pub(super) fn draw_node(
        &mut self,
        ui: &mut egui::Ui,
        node_id: &NodeID,
        view_transform: egui::Vec2,
    ) {
        let Some(track_id) = self.ui_state.selected_track else {
            return;
        };

        // Gather display data from the node and its meta.
        // Borrows end at the closing brace so we can take &mut self freely afterwards.
        let (input_names, output_names, pos, display_name) = {
            let Some(node) = self
                .project
                .get_track(&track_id)
                .and_then(|t| t.get_graph().get_node(node_id))
            else {
                return;
            };
            let Some(meta) = self
                .project_meta
                .get_track(&track_id)
                .and_then(|t| t.graph.get_node_meta(node_id))
            else {
                return;
            };
            (
                node.get_input_names(),
                node.get_output_names(),
                meta.pos,
                meta.display_name.clone(),
            )
        };

        // Calculate node geometry.
        // pos is in canvas space; view_transform converts it to screen space.
        let row_count = input_names.len().max(output_names.len()).max(1);
        let node_height =
            HEADER_HEIGHT + NODE_PADDING + PORT_ROW_HEIGHT * row_count as f32 + NODE_PADDING;
        let node_rect =
            egui::Rect::from_min_size(pos + view_transform, egui::vec2(NODE_WIDTH, node_height));
        let header_rect =
            egui::Rect::from_min_size(node_rect.min, egui::vec2(NODE_WIDTH, HEADER_HEIGHT));

        let dark_mode = ui.visuals().dark_mode;
        let painter = ui.painter();

        // Draw the node background
        let node_stroke = if self.ui_state.selected_node == Some(*node_id) {
            egui::Stroke::new(2.0, theme::region_selected(dark_mode))
        } else {
            egui::Stroke::new(1.0, theme::border(dark_mode))
        };
        painter.rect(
            node_rect,
            egui::CornerRadius::same(6),
            theme::secondary_bg(dark_mode),
            node_stroke,
            egui::StrokeKind::Outside,
        );

        // Draw the header background and its bottom border
        painter.rect_filled(
            header_rect,
            egui::CornerRadius {
                nw: 6,
                ne: 6,
                sw: 0,
                se: 0,
            },
            theme::tertiary_bg(dark_mode),
        );
        painter.line_segment(
            [header_rect.left_bottom(), header_rect.right_bottom()],
            egui::Stroke::new(1.0, theme::border(dark_mode)),
        );

        // Draw the node name in the header
        painter.text(
            header_rect.center(),
            egui::Align2::CENTER_CENTER,
            display_name.as_str(),
            egui::FontId::proportional(13.0),
            theme::primary_fg(dark_mode),
        );

        // Draw the input/output ports
        draw_ports(painter, node_rect, &input_names, &output_names, dark_mode);

        // Handle node gestures (click to select, drag to move)
        let response = ui.allocate_rect(node_rect, Sense::click_and_drag());
        self.apply_node_gesture(ui, response, node_id, &track_id);

        // Handle output port drag to create ghost edges (allocated after node so ports take priority)
        self.handle_output_port_drags(ui, node_id, node_rect, output_names.len());
        // Handle input port drag to re-route existing edges
        self.handle_input_port_drags(ui, node_id, &track_id, node_rect, input_names.len());
    }

    fn apply_node_gesture(
        &mut self,
        ui: &mut egui::Ui,
        response: egui::Response,
        node_id: &NodeID,
        track_id: &TrackID,
    ) {
        // Click or drag to select the node
        if response.clicked() || response.dragged() {
            self.ui_state.set_selected_node(*node_id);
        }

        // Drag to move the node; suppress when a ghost edge drag is in progress
        if response.dragged()
            && self.ui_state.node_graph_state.ghost_edge.is_none()
            && let Some(meta) = self
                .project_meta
                .get_track_mut(track_id)
                .and_then(|t| t.graph.get_node_meta_mut(node_id))
        {
            meta.pos += response.drag_delta();
        }

        // Delete the node if delete or backspace key is pressed
        // Check for the delete key input
        if self.ui_state.selected_node == Some(*node_id) && ui.ui_contains_pointer() {
            let delete = ui.input(|i| i.key_pressed(egui::Key::Delete));
            let backspace = ui.input(|i| i.key_pressed(egui::Key::Backspace));

            if delete || backspace {
                // Remove the note from the region
                self.remove_node(track_id, node_id);
                self.ui_state.selected_note = None;
            }
        }
    }

    fn handle_output_port_drags(
        &mut self,
        ui: &mut egui::Ui,
        node_id: &NodeID,
        node_rect: egui::Rect,
        output_count: usize,
    ) {
        for current_row in 0..output_count {
            let (port_center, port_resp) =
                edge_drag_hitbox(ui, node_rect, current_row, node_rect.max.x);

            // Change the cursor to a crosshair when hovering the output port
            if port_resp.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
            }

            // If the drag has started, create a new ghost edge starting from this port
            if port_resp.drag_started() {
                let mouse_pos = ui
                    .input(|inp| inp.pointer.hover_pos())
                    .unwrap_or(port_center);
                self.ui_state.node_graph_state.ghost_edge =
                    Some(((*node_id, current_row), mouse_pos));
            }

            // Update the position of the ghost edge
            if port_resp.dragged()
                && let Some(pos) = ui.input(|inp| inp.pointer.hover_pos())
                && let Some(ghost) = &mut self.ui_state.node_graph_state.ghost_edge
            {
                ghost.1 = pos;
            }
        }
    }

    fn handle_input_port_drags(
        &mut self,
        ui: &mut egui::Ui,
        node_id: &NodeID,
        track_id: &TrackID,
        node_rect: egui::Rect,
        input_count: usize,
    ) {
        for current_row in 0..input_count {
            let (port_center, port_resp) =
                edge_drag_hitbox(ui, node_rect, current_row, node_rect.min.x);

            // Change the cursor to a crosshair when hovering the input port
            if port_resp.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
            }

            // If the drag has started, get the edge pointing toward the input port and create a ghost edge
            if port_resp.drag_started() {
                // Find the edge connected to this input port
                let found = self.project.get_track(track_id).and_then(|t| {
                    t.get_graph()
                        .get_edges()
                        .iter()
                        .find(|(_, _, to_id, in_idx)| to_id == node_id && *in_idx == current_row)
                        .copied()
                });
                if let Some(edge) = found {
                    let (from_id, out_idx, _, _) = edge;
                    let mouse_pos = ui
                        .input(|inp| inp.pointer.hover_pos())
                        .unwrap_or(port_center);
                    self.ui_state.node_graph_state.ghost_edge =
                        Some(((from_id, out_idx), mouse_pos));
                    self.ui_state.node_graph_state.dragged_edge = Some(edge);
                }
            }

            // Update the position of the ghost edge
            if port_resp.dragged()
                && let Some(pos) = ui.input(|inp| inp.pointer.hover_pos())
                && let Some(ghost) = &mut self.ui_state.node_graph_state.ghost_edge
            {
                ghost.1 = pos;
            }
        }
    }
}

fn edge_drag_hitbox(
    ui: &mut egui::Ui,
    node_rect: egui::Rect,
    current_row: usize,
    port_x: f32,
) -> (egui::Pos2, egui::Response) {
    let y = calc_port_y(node_rect, current_row);
    let port_center = egui::pos2(port_x, y);
    let hit = PORT_RADIUS * 2.5;
    let port_rect = egui::Rect::from_center_size(port_center, egui::vec2(hit, hit));
    (
        port_center,
        ui.allocate_rect(port_rect, egui::Sense::drag()),
    )
}
