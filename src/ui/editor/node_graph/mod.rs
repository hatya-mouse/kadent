mod add_node;
mod edge;
mod node;

use crate::{colors, kasl_node::KaslNode, ui::EditorUi};
use eframe::egui;
use knodiq_engine::{
    graph::node_id::NodeID,
    mixer::TrackID,
    node::{
        Node,
        builtin::{AudioInputNode, AudioOutputNode, NoteInputNode},
    },
};

pub(super) struct NodeSnapshot {
    pub id: NodeID,
    pub label: String,
    pub input_names: Vec<String>,
    pub output_names: Vec<String>,
}

impl EditorUi {
    pub(in crate::ui) fn node_graph(&mut self, ui: &mut egui::Ui) {
        let Some((track_id, _)) = self.ui_state.selected_region else {
            ui.centered_and_justified(|ui| {
                ui.label("Select a region to view its node graph");
            });
            return;
        };

        let Some(snapshots) = self.collect_node_snapshots(track_id) else {
            return;
        };
        let edges = self.collect_graph_edges(track_id);
        let (input_id, output_id) = self.graph_terminals(track_id);

        self.ensure_node_positions(track_id, input_id, output_id, &snapshots);

        let rect = ui.available_rect_before_wrap();
        let (bg_response, painter) =
            ui.allocate_painter(rect.size(), egui::Sense::drag().union(egui::Sense::click()));

        painter.rect_filled(rect, 0.0, colors::secondary_bg(ui.visuals().dark_mode));

        let pan = self
            .project_meta
            .get_track(&track_id)
            .map(|m| m.node_graph.pan_offset)
            .unwrap_or_default();

        for edge in &edges {
            self.draw_graph_edge(&painter, track_id, edge, pan, rect.min);
        }

        // Draw the dragged edge is exists
        if let Some(ghost_edge) = self.ui_state.node_graph_state.ghost_edge {
            self.draw_dragged_edge(
                &painter,
                track_id,
                ghost_edge.0,
                ghost_edge.1,
                pan,
                rect.min,
            );
        }

        let mut any_node_dragged = false;
        for snapshot in &snapshots {
            let Some(pos) = self
                .project_meta
                .get_track(&track_id)
                .and_then(|m| m.node_graph.get_node_pos(snapshot.id))
            else {
                continue;
            };
            let dragged =
                self.draw_and_interact_node(ui, &painter, track_id, snapshot, pos, pan, rect.min);
            if dragged {
                any_node_dragged = true;
            }
        }

        if !any_node_dragged
            && bg_response.dragged()
            && let Some(track_meta) = self.project_meta.get_track_mut(&track_id)
        {
            track_meta.node_graph.pan_offset += bg_response.drag_delta();
        }

        // Save the canvas-space position of a right-click for use when the menu action fires
        if bg_response.secondary_clicked()
            && let Some(screen_pos) = bg_response.interact_pointer_pos()
        {
            self.ui_state.node_graph_add_pos = Some(egui::pos2(
                screen_pos.x - rect.min.x - pan.x,
                screen_pos.y - rect.min.y - pan.y,
            ));
        }

        // Context menu
        let mut do_add: Option<add_node::AddableNodeKind> = None;
        bg_response.context_menu(|ui| {
            for &kind in add_node::AddableNodeKind::all() {
                if ui.button(kind.label()).clicked() {
                    do_add = Some(kind);
                    ui.close();
                }
            }
        });

        if let Some(kind) = do_add
            && let Some(canvas_pos) = self.ui_state.node_graph_add_pos
        {
            let node = kind.create(&self.project_meta, &self.project_dir);
            self.add_node_to_graph(track_id, node, canvas_pos);
        }
    }

    fn collect_node_snapshots(&self, track_id: TrackID) -> Option<Vec<NodeSnapshot>> {
        let track = self.project.tracks.get(&track_id)?;
        let graph = track.get_graph();
        let input_id = graph.get_input_id();
        let output_id = graph.get_output_id();

        let snapshots = graph
            .get_node_map()
            .iter()
            .map(|(id, node)| NodeSnapshot {
                id: *id,
                label: node_label(node.as_ref(), *id == input_id, *id == output_id),
                input_names: node.get_input_names(),
                output_names: node.get_output_names(),
            })
            .collect();

        Some(snapshots)
    }

    fn collect_graph_edges(&self, track_id: TrackID) -> Vec<(NodeID, usize, NodeID, usize)> {
        self.project
            .tracks
            .get(&track_id)
            .map(|track| track.get_graph().get_edges().clone())
            .unwrap_or_default()
    }

    fn graph_terminals(&self, track_id: TrackID) -> (NodeID, NodeID) {
        self.project
            .tracks
            .get(&track_id)
            .map(|track| {
                let g = track.get_graph();
                (g.get_input_id(), g.get_output_id())
            })
            .unwrap_or_default()
    }

    fn ensure_node_positions(
        &mut self,
        track_id: TrackID,
        input_id: NodeID,
        output_id: NodeID,
        snapshots: &[NodeSnapshot],
    ) {
        const SPACING_X: f32 = 220.0;
        const START_X: f32 = 50.0;
        const START_Y: f32 = 150.0;

        let mut ordered: Vec<NodeID> = Vec::with_capacity(snapshots.len());
        ordered.push(input_id);
        for s in snapshots {
            if s.id != input_id && s.id != output_id {
                ordered.push(s.id);
            }
        }
        ordered.push(output_id);

        if let Some(track_meta) = self.project_meta.get_track_mut(&track_id) {
            for (i, &node_id) in ordered.iter().enumerate() {
                let default = egui::pos2(START_X + i as f32 * SPACING_X, START_Y);
                track_meta.node_graph.ensure_node_pos(node_id, default);
            }
        }
    }
}

fn node_label(node: &dyn Node, is_input: bool, is_output: bool) -> String {
    if is_input {
        if node.as_any().is::<NoteInputNode>() {
            return "Note Input".to_string();
        }
        if node.as_any().is::<AudioInputNode>() {
            return "Audio Input".to_string();
        }
    }
    if is_output && node.as_any().is::<AudioOutputNode>() {
        return "Audio Output".to_string();
    }
    if let Some(kasl) = node.as_any().downcast_ref::<KaslNode>() {
        return kasl
            .get_file_path()
            .and_then(|p| p.file_stem())
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "KASL Node".to_string());
    }
    "Node".to_string()
}
