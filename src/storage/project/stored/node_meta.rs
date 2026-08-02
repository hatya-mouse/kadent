use crate::core::metadata::{NodeMeta, NodeType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct StoredNodeMeta {
    node_type: NodeType,
    display_name: String,
    pos: (f32, f32),
}

impl StoredNodeMeta {
    pub fn from_node_meta(node_meta: &NodeMeta) -> Self {
        Self {
            node_type: node_meta.node_type.clone(),
            display_name: node_meta.display_name.clone(),
            pos: (node_meta.pos.x, node_meta.pos.y),
        }
    }

    pub fn to_node_meta(&self) -> NodeMeta {
        NodeMeta {
            node_type: self.node_type.clone(),
            display_name: self.display_name.clone(),
            pos: eframe::egui::pos2(self.pos.0, self.pos.1),
        }
    }
}
