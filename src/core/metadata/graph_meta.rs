use crate::core::audio_engine::graph::{Graph, node_id::NodeID};
use crate::core::metadata::NodeMeta;
use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub(crate) struct GraphMeta {
    /// The positions of nodes in canvas space.
    pub(crate) nodes: HashMap<NodeID, NodeMeta>,
}

impl GraphMeta {
    pub(crate) fn from_graph(graph: &Graph) -> Self {
        let mut nodes = HashMap::new();
        for (node_id, node) in graph.get_node_map() {
            nodes.insert(*node_id, NodeMeta::from_node(&**node));
        }
        Self { nodes }
    }

    pub(crate) fn get_node_meta(&self, node_id: &NodeID) -> Option<&NodeMeta> {
        self.nodes.get(node_id)
    }

    pub(crate) fn get_node_meta_mut(&mut self, node_id: &NodeID) -> Option<&mut NodeMeta> {
        self.nodes.get_mut(node_id)
    }

    pub(crate) fn set_node_meta(&mut self, node_id: NodeID, node_meta: NodeMeta) {
        self.nodes.insert(node_id, node_meta);
    }

    pub(crate) fn remove_node(&mut self, node_id: &NodeID) {
        self.nodes.remove(node_id);
    }
}
