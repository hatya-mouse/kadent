use crate::core::audio_engine::{graph::node_id::NodeID, mixer::TrackID};
use crate::core::kasl_node::KaslNode;
use crate::ui::editor::EditorUi;

impl EditorUi {
    pub(crate) fn compile_kasl_node(&mut self, track_id: &TrackID, node_id: &NodeID) {
        let Some(track) = self.state.project.data.get_track_mut(track_id) else {
            return;
        };
        let Some(node) = track.get_graph_mut().get_node_mut(node_id) else {
            return;
        };
        let Some(kasl_node) = node.as_any_mut().downcast_mut::<KaslNode>() else {
            return;
        };

        let export_ctx = &self.state.project.meta.export_ctx;
        if let Err(err) = kasl_node.compile(export_ctx) {
            eprintln!("Error compiling KaslNode: {}", err);
        } else {
            self.state.actions.modified_project();
        }
    }
}
