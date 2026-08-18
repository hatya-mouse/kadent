mod layout;
mod leaf;
mod split;

pub(crate) use layout::{PanelNode, PanelView, SplitDir};
use uuid::Uuid;

use crate::ui::EditorState;
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
        self.render_node(ui, &mut layout, rect);

        // Remove code buffers for panels that no longer exist in the layout tree
        let active_ids = collect_panel_ids(&layout);
        self.views
            .code_editor
            .code_buffers
            .retain(|id, _| active_ids.contains(id));

        self.layout = layout;
    }

    fn render_node(&mut self, ui: &mut egui::Ui, node: &mut PanelNode, rect: egui::Rect) {
        let split_action = match node {
            PanelNode::Leaf(view, id) => self.render_leaf(ui, view, *id, rect),
            _ => None,
        };

        let collapse_keep_first = match node {
            PanelNode::Split {
                dir,
                ratio,
                first,
                second,
            } => self.render_split(ui, *dir, ratio, first, second, rect),
            _ => None,
        };

        if let Some(action) = split_action {
            let (current, current_id) = match std::mem::take(node) {
                PanelNode::Leaf(current, current_id) => (current, current_id),
                _ => unreachable!(),
            };
            let (a, b) = if action.new_panel_first {
                (
                    PanelNode::Leaf(PanelView::Timeline, Uuid::new_v4()),
                    PanelNode::Leaf(current, current_id),
                )
            } else {
                (
                    PanelNode::Leaf(current, current_id),
                    PanelNode::Leaf(PanelView::Timeline, Uuid::new_v4()),
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

/// Recursively collects the ids.
fn collect_panel_ids(node: &PanelNode) -> HashSet<Uuid> {
    match node {
        PanelNode::Leaf(_, node_id) => {
            let mut set = HashSet::new();
            set.insert(*node_id);
            set
        }
        PanelNode::Split { first, second, .. } => {
            let mut ids = collect_panel_ids(first);
            ids.extend(collect_panel_ids(second));
            ids
        }
    }
}
