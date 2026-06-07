mod leaf;
mod split;

use crate::ui::workspaces::EditorUi;
use crate::ui_state::panel_layout::{PanelNode, PanelView, SplitDir};
use eframe::egui;

struct SplitAction {
    dir: SplitDir,
    ratio: f32,
    new_panel_first: bool,
}

impl EditorUi {
    /// Renders the panel layout by recursively drawing the each nodes.
    pub(in crate::ui) fn render_panels(&mut self, ui: &mut egui::Ui, rect: egui::Rect) {
        // Extract the layout tree to avoid a simultaneous borrow of self
        let mut layout = std::mem::take(&mut self.ui_state.panel_layout);
        render_node(ui, &mut layout, rect, self);
        self.ui_state.panel_layout = layout;
    }
}

fn render_node(ui: &mut egui::Ui, node: &mut PanelNode, rect: egui::Rect, editor: &mut EditorUi) {
    // Each arm is a separate match so borrows don't overlap

    let split_action = match node {
        PanelNode::Leaf(view) => leaf::render_leaf(ui, view, rect, editor),
        _ => None,
    };

    let collapse_keep_first = match node {
        PanelNode::Split {
            dir,
            ratio,
            first,
            second,
        } => split::render_split(ui, *dir, ratio, first, second, rect, editor),
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
