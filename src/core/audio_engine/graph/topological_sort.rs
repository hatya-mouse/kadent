use crate::core::audio_engine::graph::{
    Graph, InputSource, OutputKey, error::GraphError, node_id::NodeID,
};
use std::collections::HashMap;

#[derive(PartialEq)]
enum SortState {
    Unvisited,
    Visiting,
    Visited,
}

impl Graph {
    /// Sorts the processing graph in a topological order. Returns error if a cycle is found.
    /// Omits the input and output node.
    pub(crate) fn sort_graph(&mut self) -> Result<(), GraphError> {
        // Rebuild adjacency from edges so the sort reflects the current graph structure
        self.adjacency.clear();
        for (to, input_source) in &self.input_sources {
            if let &InputSource::Edge(OutputKey(from_node, _)) = input_source {
                self.adjacency.entry(from_node).or_default().push(to.0);
            }
        }

        // Create visited and sorted array
        let mut states: HashMap<NodeID, SortState> = self
            .nodes
            .keys()
            .map(|k| (*k, SortState::Unvisited))
            .collect();
        let mut sorted: Vec<NodeID> = Vec::new();

        // Sort the graph recursively
        for node in self.nodes.keys().copied().collect::<Vec<NodeID>>() {
            let state = states.get(&node);
            if state.is_none_or(|s| s == &SortState::Unvisited) {
                self.search_recursively(node, &mut states, &mut sorted)?;
            } else if state.is_some_and(|s| s == &SortState::Visiting) {
                return Err(GraphError::NodeCycle(node));
            }
        }

        // Remove the input node and the output node from the sorted vector
        sorted.retain(|n| n != &self.input_id && n != &self.output_id);
        // Reverse the sorted vector
        sorted.reverse();
        self.sorted_nodes = sorted;

        Ok(())
    }

    /// Sorts the graph recursively using depth-first search algorithm.
    fn search_recursively(
        &mut self,
        node: NodeID,
        states: &mut HashMap<NodeID, SortState>,
        sorted: &mut Vec<NodeID>,
    ) -> Result<(), GraphError> {
        // Mark the node as visiting
        states.insert(node, SortState::Visiting);
        // Check the neighboring nodes
        let neighbors = self.adjacency.get(&node).cloned().unwrap_or_default();
        for neighbor in neighbors {
            let state = states.get(&neighbor);
            if state.is_none_or(|s| s == &SortState::Unvisited) {
                self.search_recursively(neighbor, states, sorted)?;
            } else if state.is_some_and(|s| s == &SortState::Visiting) {
                return Err(GraphError::NodeCycle(neighbor));
            }
        }
        // Mark the node as visited
        states.insert(node, SortState::Visited);
        // Add the current node after searching all of its neighboring nodes
        sorted.push(node);
        Ok(())
    }
}
