use crate::storage::project::stored::node::StoredNode;
use kadent_engine::graph::{Graph, InputSource, automation::KeyframeManager, node_id::NodeID};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Mirror of `Graph` for persistence.
/// `next_node_id` isn't stored but it's recomputed from the loaded node IDs.
#[derive(Serialize, Deserialize)]
pub(crate) struct StoredGraph {
    pub nodes: HashMap<NodeID, StoredNode>,
    pub input_sources: HashMap<(NodeID, usize), InputSource>,
    pub keyframe_manager: KeyframeManager,
    pub input_id: NodeID,
    pub output_id: NodeID,
}

impl StoredGraph {
    pub fn from_graph(graph: &Graph) -> Self {
        let nodes = graph
            .get_node_map()
            .iter()
            .filter_map(|(id, node)| {
                StoredNode::from_node(node.as_ref()).map(|stored| (*id, stored))
            })
            .collect();

        Self {
            nodes,
            input_sources: graph.input_sources.clone(),
            keyframe_manager: graph.keyframe_manager.clone(),
            input_id: graph.get_input_id(),
            output_id: graph.get_output_id(),
        }
    }

    pub fn to_graph(&self) -> Graph {
        let mut graph = Graph::default();
        for (id, stored_node) in &self.nodes {
            graph.add_node_with_id(*id, stored_node.to_node());
        }
        graph.set_input_id(self.input_id);
        graph.set_output_id(self.output_id);
        graph.input_sources = self.input_sources.clone();

        restore_next_node_id(&mut graph);
        graph
    }
}

fn restore_next_node_id(graph: &mut Graph) {
    let next_id = graph
        .get_node_map()
        .keys()
        .map(|id| id.0)
        .max()
        .map(|m| m + 1)
        .unwrap_or(0);
    graph.set_next_node_id(next_id);
}
