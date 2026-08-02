use crate::{core::metadata::GraphMeta, storage::project::stored::node_meta::StoredNodeMeta};
use kadent_engine::graph::node_id::NodeID;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct StoredGraphMeta {
    node_metas: HashMap<NodeID, StoredNodeMeta>,
}

impl StoredGraphMeta {
    pub fn from_graph_meta(graph_meta: &GraphMeta) -> Self {
        Self {
            node_metas: graph_meta
                .nodes
                .iter()
                .map(|(node_id, node)| (*node_id, StoredNodeMeta::from_node_meta(node)))
                .collect(),
        }
    }

    pub fn to_graph_meta(&self) -> GraphMeta {
        let mut graph_meta = GraphMeta::default();
        for (node_id, node) in &self.node_metas {
            graph_meta.set_node_meta(*node_id, node.to_node_meta());
        }
        graph_meta
    }
}
