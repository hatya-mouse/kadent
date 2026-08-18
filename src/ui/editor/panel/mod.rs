mod leaf;
mod split;

use crate::ui::{
    EditorState,
    editor::state::{PanelNode, PanelView, SplitDir},
};
use eframe::egui;
use std::collections::HashSet;

struct SplitAction {
    dir: SplitDir,
    ratio: f32,
    new_panel_first: bool,
}

impl EditorState {
    /// Renders the panel layout by recursively drawing the each nodes.
    pub(in crate::ui) fn render_panels(&mut self, ui: &mut egui::Ui, rect: egui::Rect) {
        // Extract the layout tree to avoid a simultaneous borrow of self
        let mut layout = std::mem::take(&mut self.layout);
        self.render_node(ui, &mut layout, rect, ui.id().with("panel_root"));

        // Remove code buffers for panels that no longer exist in the layout tree
        let active_ids = collect_code_editor_ids(&layout, ui.id().with("panel_root"));
        self.views.code_editor
            .code_buffers
            .retain(|id, _| active_ids.contains(id));

        self.layout = layout;
    }

    fn render_node(
        &mut self,
        ui: &mut egui::Ui,
        node: &mut PanelNode,
        rect: egui::Rect,
        node_id: egui::Id,
    ) {
        let split_action = match node {
            PanelNode::Leaf(view) => self.render_leaf(ui, view, rect, node_id),
            _ => None,
        };

        let collapse_keep_first = match node {
            PanelNode::Split {
                dir,
                ratio,
                first,
                second,
            } => self.render_split(ui, *dir, ratio, first, second, rect, node_id),
            _ => None,
        };

        if let Some(action) = split_action {
            let current = match std::mem::take(node) {
                PanelNode::Leaf(v) => v,
                _ => unreachable!(),
            };
            let (a, b) = if action.new_panel_first {
                (
                    PanelNode::Leaf(PanelView::Timeline),
                    PanelNode::Leaf(current),
                )
            } else {
                (
                    PanelNode::Leaf(current),
                    PanelNode::Leaf(PanelView::Timeline),
                )
            };
            *node = PanelNode::Split {
                dir: action.dir,
                ratio: action.ratio,
                first: Box::new(a),
                second: Box::new(b),
            };
        } else if let Some(keep_first) = collapse_keep_first {
            *node = match std::mem::take(node) {
                PanelNode::Split { first, second, .. } => {
                    if keep_first {
                        *first
                    } else {
                        *second
                    }
                }
                other => other,
            };
        }
    }
}

/// Recursively collects the stable panel IDs of all CodeEditor leaves in the tree.
fn collect_code_editor_ids(node: &PanelNode, node_id: egui::Id) -> HashSet<egui::Id> {
    match node {
        PanelNode::Leaf(PanelView::CodeEditor) => {
            let mut set = HashSet::new();
            set.insert(node_id);
            set
        }
        PanelNode::Leaf(_) => HashSet::new(),
        PanelNode::Split { first, second, .. } => {
            let mut ids = collect_code_editor_ids(first, node_id.with("first"));
            ids.extend(collect_code_editor_ids(second, node_id.with("second")));
            ids
        }
    }
}
