use crate::ui::{EditorState, components::centered_text::centered_text};
use eframe::egui;
use kadent_engine::node::builtin::AutomationNode;

impl EditorState {
    pub(in crate::ui::editor) fn automation(&mut self, ui: &mut egui::Ui) {
        let (Some(track_id), Some(node_id)) = (self.selection.track_id(), self.selection.node_id())
        else {
            centered_text(ui, "No Automation Node Selected");
            return;
        };
        let Some(automation_node) = self
            .project
            .data
            .get_track_mut(&track_id)
            .and_then(|track| track.get_graph_mut().get_node_mut(&node_id))
            .and_then(|node| node.as_any_mut().downcast_mut::<AutomationNode>())
        else {
            centered_text(ui, "No Automation Node Selected");
            return;
        };
    }
}
