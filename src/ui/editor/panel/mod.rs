mod layout;
mod leaf;
mod split;

pub(crate) use layout::{PanelNode, PanelView, SplitDir};

use crate::ui::editor::EditorUi;
use eframe::egui;
use uuid::Uuid;

struct SplitAction {
    dir: SplitDir,
    ratio: f32,
    new_panel_first: bool,
}

impl EditorUi {
    /// Renders the panel layout by recursively drawing the each nodes.
    pub(in crate::ui) fn render_panels(&mut self, ui: &mut egui::Ui, rect: egui::Rect) {
        // Extract the layout tree to avoid a simultaneous borrow of self
        let mut layout = std::mem::take(&mut self.layout);
        self.render_node(ui, &mut layout, rect);

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
            let (retained, removed) = match std::mem::take(node) {
                PanelNode::Split { first, second, .. } => {
                    if keep_first {
                        (*first, Some(*second))
                    } else {
                        (*second, Some(*first))
                    }
                }
                other => (other, None),
            };
            *node = retained;

            // Remove the state of the removed panel(s) from the view states
            if let Some(removed) = removed {
                let removed_ids = collect_panel_ids(&removed);
                self.views.remove_panel_states(&removed_ids);
            }
        }
    }
}

/// Recursively collects the ids.
fn collect_panel_ids(node: &PanelNode) -> Vec<Uuid> {
    match node {
        PanelNode::Leaf(_, node_id) => {
            vec![*node_id]
        }
        PanelNode::Split { first, second, .. } => {
            let mut ids = collect_panel_ids(first);
            ids.extend(collect_panel_ids(second));
            ids
        }
    }
}
