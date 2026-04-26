use crate::{colors, metadata::ProjectMeta, ui::EditorUi, ui_state::editor_state::EditorUiState};
use eframe::egui::{self, Sense};
use knodiq_engine::{graph::node_id::NodeID, mixer::TrackID};

const NODE_WIDTH: f32 = 180.0;
const HEADER_HEIGHT: f32 = 28.0;
const PORT_ROW_HEIGHT: f32 = 22.0;
const PORT_RADIUS: f32 = 5.0;
const PADDING: f32 = 8.0;

impl EditorUi {
    pub(super) fn draw_node(&mut self, ui: &mut egui::Ui, node_id: &NodeID) {
        let Some(track_id) = self.ui_state.selected_track else {
            return;
        };

        let (input_len, output_len, pos, display_name) = {
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
                node.get_input_len(),
                node.get_output_len(),
                meta.pos,
                meta.display_name.clone(),
            )
        };

        let row_count = input_len.max(output_len).max(1);
        let node_height = HEADER_HEIGHT + PORT_ROW_HEIGHT * row_count as f32 + PADDING;
        let node_rect = egui::Rect::from_min_size(pos, egui::vec2(NODE_WIDTH, node_height));
        let header_rect =
            egui::Rect::from_min_size(node_rect.min, egui::vec2(NODE_WIDTH, HEADER_HEIGHT));

        let painter = ui.painter();

        // Draw the background of the node
        painter.rect(
            node_rect,
            egui::CornerRadius::same(6),
            colors::secondary_bg(ui.visuals().dark_mode),
            egui::Stroke::new(1.0, colors::border(ui.visuals().dark_mode)),
            egui::StrokeKind::Outside,
        );

        // Draw the header
        painter.rect_filled(
            header_rect,
            egui::CornerRadius {
                nw: 6,
                ne: 6,
                sw: 0,
                se: 0,
            },
            colors::tertiary_bg(ui.visuals().dark_mode),
        );
        painter.line_segment(
            [header_rect.left_bottom(), header_rect.right_bottom()],
            egui::Stroke::new(1.0, colors::border(ui.visuals().dark_mode)),
        );
        painter.text(
            header_rect.center(),
            egui::Align2::CENTER_CENTER,
            display_name.as_str(),
            egui::FontId::proportional(13.0),
            colors::primary_fg(ui.visuals().dark_mode),
        );

        let response = ui.allocate_rect(node_rect, Sense::click_and_drag());
        Self::apply_node_gesture(
            response,
            node_id,
            &track_id,
            &mut self.project_meta,
            &mut self.ui_state,
        );
    }

    fn apply_node_gesture(
        response: egui::Response,
        node_id: &NodeID,
        track_id: &TrackID,
        project_meta: &mut ProjectMeta,
        ui_state: &mut EditorUiState,
    ) {
        if response.clicked() || response.dragged() {
            ui_state.set_selected_node(*node_id);
        }

        if response.dragged()
            && let Some(meta) = project_meta
                .get_track_mut(track_id)
                .and_then(|t| t.graph.get_node_meta_mut(node_id))
        {
            meta.pos += response.drag_delta();
        }
    }
}
