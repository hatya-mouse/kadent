use kadent_engine::{graph::node_id::NodeID, mixer::TrackID};

use crate::{core::kasl_node::KaslNode, ui::workspaces::EditorUi};

impl EditorUi {
    pub(crate) fn compile_kasl_node(&mut self, track_id: &TrackID, node_id: &NodeID) {
        let Some(track) = self.project.get_track_mut(track_id) else {
            return;
        };
        let Some(node) = track.get_graph_mut().get_node_mut(node_id) else {
            return;
        };
        let Some(kasl_node) = node.as_any_mut().downcast_mut::<KaslNode>() else {
            return;
        };

        if let Err(err) = kasl_node.compile() {
            eprintln!("Error compiling KaslNode: {}", err);
        } else {
            self.modified_project();
        }
    }
}
