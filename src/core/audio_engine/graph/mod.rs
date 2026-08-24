pub(crate) mod error;
pub(crate) mod node_id;
mod topological_sort;

use crate::core::audio_engine::{
    data_types::PlaybackContext,
    graph::{error::GraphError, node_id::NodeID},
    node::Node,
    timing::TempoMap,
};
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct InputKey(pub(crate) NodeID, pub(crate) usize);

#[derive(Debug, Clone)]
pub(crate) enum InputSource {
    Edge(NodeID, usize),
    Keyframe,
    Zero,
}

#[derive(Default, Clone)]
pub(crate) struct Graph {
    // --- GRAPH STRUCTURE ---
    nodes: HashMap<NodeID, Box<dyn Node>>,
    input_sources: HashMap<InputKey, InputSource>,
    adjacency: HashMap<NodeID, Vec<NodeID>>,
    input_id: NodeID,
    output_id: NodeID,

    // --- PROCESSING DATA ---
    sorted_nodes: Vec<NodeID>,
    /// Allocated buffers for the output of each node, which are used as input for the connected nodes.
    output_buffers: HashMap<(NodeID, usize), Vec<u8>>,
    /// Allocated buffers for the calculated keyframe values.
    keyframe_buffers: HashMap<InputKey, Vec<u8>>,
    /// Stores pointers to the input buffers of each node, which may be the same as the node output buffers of the connected node.
    node_inputs: HashMap<NodeID, Vec<*const u8>>,
    /// Stores pointers to the output buffers of each node.
    node_outputs: HashMap<NodeID, Vec<*mut u8>>,
    zero_buffer: Vec<u8>,

    // --- MISC ---
    next_node_id: u64,
}

impl Graph {
    // --- INITIALIZATION ---

    /// Creates a new Graph instance with the given input and output node..
    pub(crate) fn new(input_node: Box<dyn Node>, output_node: Box<dyn Node>) -> Self {
        let mut graph = Graph::default();
        // Register the input and output nodes
        let input_id = graph.add_node(input_node);
        let output_id = graph.add_node(output_node);
        graph.input_id = input_id;
        graph.output_id = output_id;
        // Return the newly created graph
        graph
    }

    /// Creates a new graph with the given nodes, input_sources, input_id, and output_id.
    pub(crate) fn with_nodes(
        nodes: HashMap<NodeID, Box<dyn Node>>,
        input_sources: HashMap<InputKey, InputSource>,
        input_id: NodeID,
        output_id: NodeID,
        next_node_id: u64,
    ) -> Self {
        Graph {
            nodes,
            input_sources,
            input_id,
            output_id,
            next_node_id,
            ..Default::default()
        }
    }

    // --- ID GENERATION ---

    /// Generates a new NodeID which is unique inside the graph.
    fn generate_node_id(&mut self) -> NodeID {
        let id = NodeID(self.next_node_id);
        self.next_node_id += 1;
        id
    }

    // --- NODE GETTING ---

    pub(crate) fn get_input_sources(&self) -> &HashMap<InputKey, InputSource> {
        &self.input_sources
    }

    pub(crate) fn get_input_id(&self) -> NodeID {
        self.input_id
    }

    pub(crate) fn get_output_id(&self) -> NodeID {
        self.output_id
    }

    pub(crate) fn get_node_map(&self) -> &HashMap<NodeID, Box<dyn Node>> {
        &self.nodes
    }

    pub(crate) fn get_node_map_mut(&mut self) -> &mut HashMap<NodeID, Box<dyn Node>> {
        &mut self.nodes
    }

    pub(crate) fn get_node(&self, id: &NodeID) -> Option<&dyn Node> {
        self.nodes.get(id).map(|track| &**track)
    }

    pub(crate) fn get_node_mut(&mut self, id: &NodeID) -> Option<&mut Box<dyn Node>> {
        self.nodes.get_mut(id)
    }

    // --- NODE MANIPULATION ---

    /// Adds a new node to the graph, and returns the newly generated node ID.
    pub(crate) fn add_node(&mut self, mut node: Box<dyn Node>) -> NodeID {
        let id = self.generate_node_id();
        // Update the node
        node.update_type_info();
        // Insert the node to the map
        self.nodes.insert(id, node);
        id
    }

    /// Removes the node with the given NodeID from the graph.
    pub(crate) fn remove_node(&mut self, id: &NodeID) {
        // Remove the edges connected to the node
        self.input_sources
            .retain(|&InputKey(to_node, _), _| to_node != *id);
        self.input_sources.retain(|&InputKey(_, _), source| {
            if let InputSource::Edge(from_node, _) = source {
                *from_node != *id
            } else {
                true
            }
        });
        // Remove the node
        self.nodes.remove(id);
    }

    // --- EDGE MANIPULATION ---

    /// Connects the node's output to another node's input, and returns an error if the type of the output and input are not the same, or if the node is not found.
    /// This function overwrites the existing edge if it exists.
    pub(crate) fn add_edge(
        &mut self,
        from: (NodeID, usize),
        to: InputKey,
    ) -> Result<(), GraphError> {
        // Check if the type of the output and input are the same
        let output_type = self
            .nodes
            .get(&from.0)
            .and_then(|node| node.get_output_type(from.1))
            .ok_or(GraphError::OutputTypeUnavailable(from.0, from.1))?;
        let input_type = self
            .nodes
            .get(&to.0)
            .and_then(|node| node.get_input_type(to.1))
            .ok_or(GraphError::InputTypeUnavailable(to.0, to.1))?;

        if output_type != input_type {
            return Err(GraphError::NodeTypeMismatch((from.0, from.1, to.0, to.1)));
        }

        self.input_sources
            .insert(to, InputSource::Edge(from.0, from.1));
        Ok(())
    }

    /// Removes the edge from the graph.
    pub(crate) fn remove_edge(&mut self, to: &InputKey) {
        self.input_sources.remove(to);
    }

    /// Get all edges in the graph.
    ///
    /// # Return
    /// ```
    /// (from_node, from_output, to_node, to_input)
    /// ```
    pub(crate) fn get_all_edges(&self) -> Vec<(NodeID, usize, NodeID, usize)> {
        self.input_sources
            .iter()
            .filter_map(|(InputKey(to_node, to_input), source)| {
                if let InputSource::Edge(from_node, from_output) = source {
                    Some((*from_node, *from_output, *to_node, *to_input))
                } else {
                    None
                }
            })
            .collect()
    }

    // --- PLAYBACK CONTEXT UPDATING ---

    /// Sets the playback context to the new one.
    pub(crate) fn update_type_info(&mut self) {
        // Call update functions for every nodes
        for node in self.nodes.values_mut() {
            node.update_type_info();
        }
    }

    // --- GRAPH PROCESSING ---

    fn allocate_output_buffer(
        node_id: &NodeID,
        node: &dyn Node,
        output_buffers: &mut HashMap<(NodeID, usize), Vec<u8>>,
        node_outputs: &mut HashMap<NodeID, Vec<*mut u8>>,
        playback_ctx: &PlaybackContext,
    ) -> Result<(), GraphError> {
        // Ensure an output buffer exists even for nodes with no outputs
        node_outputs.entry(*node_id).or_default();
        // Create a buffer for all outputs
        for output_index in 0..node.get_output_len() {
            let output_type = node
                .get_output_type(output_index)
                .ok_or(GraphError::OutputTypeUnavailable(*node_id, output_index))?;
            let buffer = vec![0u8; output_type.actual_size(playback_ctx.buffer_size)];

            // Insert the output buffer to the output_buffers
            output_buffers.insert((*node_id, output_index), buffer);

            // Register the pointer to the buffer in the node_outputs map
            let Some(ptr) = output_buffers
                .get_mut(&(*node_id, output_index))
                .map(|b| b.as_mut_ptr())
            else {
                return Err(GraphError::OutputBufferNotFound(*node_id, output_index));
            };
            node_outputs.entry(*node_id).or_default().push(ptr);
        }

        Ok(())
    }

    /// Prepares the graph for processing. The host must call this function before start processing, or it may lead to undefined behavior.
    pub(crate) fn prepare(
        &mut self,
        tempo_map: &TempoMap,
        playback_ctx: &PlaybackContext,
    ) -> Result<(), GraphError> {
        // First sort the graph
        self.sort_graph()?;

        // Prepare the input node and allocate its output buffer
        if let Some(input_node) = self.nodes.get_mut(&self.input_id) {
            input_node
                .prepare(tempo_map, playback_ctx)
                .map_err(GraphError::NodeError)?;

            Self::allocate_output_buffer(
                &self.input_id,
                input_node.as_ref(),
                &mut self.output_buffers,
                &mut self.node_outputs,
                playback_ctx,
            )?;
        }

        // Prepare the output node as well
        if let Some(output_node) = self.nodes.get_mut(&self.output_id) {
            output_node
                .prepare(tempo_map, playback_ctx)
                .map_err(GraphError::NodeError)?;
        }

        for node_id in &self.sorted_nodes {
            if let Some(node) = self.nodes.get_mut(node_id) {
                // Call prepare function for every nodes
                node.prepare(tempo_map, playback_ctx)
                    .map_err(GraphError::NodeError)?;

                Self::allocate_output_buffer(
                    node_id,
                    node.as_ref(),
                    &mut self.output_buffers,
                    &mut self.node_outputs,
                    playback_ctx,
                )?;
            }
        }

        // Calculate the max buffer size possible and create a zero buffer
        let mut max_size = 4usize;
        for (node_id, node) in &self.nodes {
            for i in 0..node.get_input_len() {
                let type_info = node
                    .get_input_type(i)
                    .ok_or(GraphError::InputTypeUnavailable(*node_id, i))?;
                max_size = max_size.max(type_info.actual_size(playback_ctx.buffer_size));
            }
        }
        self.zero_buffer = vec![0u8; max_size];

        // Build node_inputs from edges
        for (to, input_source) in &self.input_sources {
            if let &InputSource::Edge(from_node, from_output) = input_source {
                let Some(ptr) = self
                    .output_buffers
                    .get(&(from_node, from_output))
                    .map(|b| b.as_ptr())
                else {
                    return Err(GraphError::OutputBufferNotFound(from_node, from_output));
                };

                // Insert the pointer to the input buffer of the node
                self.node_inputs.entry(to.0).or_insert_with(|| {
                    vec![self.zero_buffer.as_ptr(); self.nodes[&to.0].get_input_len()]
                })[to.1] = ptr;
            }
        }

        // Allocate the buffers for the keyframe inputs
        self.keyframe_buffers.clear();
        let zero_ptr = self.zero_buffer.as_ptr();
        let all_node_ids: Vec<NodeID> = self
            .sorted_nodes
            .iter()
            .chain(std::iter::once(&self.output_id))
            .copied()
            .collect();
        for node_id in all_node_ids {
            let input_len = self.nodes.get(&node_id).map_or(0, |n| n.get_input_len());
            let mut input_ptrs = vec![zero_ptr; input_len];

            for (input_index, input_ptr) in input_ptrs.iter_mut().enumerate() {
                let key = InputKey(node_id, input_index);

                // Get the input source of the input
                match self.input_sources.get(&key) {
                    Some(InputSource::Edge(from_node, from_output)) => {
                        if let Some(buffer) = self.output_buffers.get(&(*from_node, *from_output)) {
                            *input_ptr = buffer.as_ptr();
                        }
                    }
                    Some(InputSource::Keyframe) => {
                        let input_type = self.nodes[&node_id]
                            .get_input_type(input_index)
                            .ok_or(GraphError::InputTypeUnavailable(node_id, input_index))?;

                        // Allocate the keyframe buffer for the input based on the input type and the buffer size
                        let buf_size = input_type.actual_size(playback_ctx.buffer_size);
                        let keyframe_buf = vec![0u8; buf_size];

                        *input_ptr = keyframe_buf.as_ptr();
                        self.keyframe_buffers.insert(key, keyframe_buf);
                    }
                    Some(InputSource::Zero) => {
                        *input_ptr = zero_ptr;
                    }
                    _ => {}
                }
            }

            self.node_inputs.insert(node_id, input_ptrs);
        }

        Ok(())
    }

    /// Processes the graph in the sorted order and writes the result in the output pointer.
    /// The host must pass the audio context which is as the same as the one given in the `set_audio_ctx` function.
    pub(crate) fn process(
        &mut self,
        inputs: &[*const u8],
        outputs: &[*mut u8],
        playhead: usize,
        playback_ctx: &PlaybackContext,
    ) {
        // Get the pointer to the output buffer of the input node
        let Some(output_buffers) = self.get_output_ptr(&self.input_id) else {
            return;
        };
        let Some(input_node) = self.nodes.get_mut(&self.input_id) else {
            return;
        };
        // Process the input node
        input_node.process(inputs, &output_buffers, playhead, playback_ctx);

        for node_id in self.sorted_nodes.clone() {
            // Get the pointer to the input buffer of the node
            let Some(input_buffers) = self.get_input_ptr(&node_id) else {
                return;
            };
            // Get the pointer to the output buffer of the node
            let Some(output_buffers) = self.get_output_ptr(&node_id) else {
                return;
            };

            // Pass the pointers and process
            if let Some(node) = self.nodes.get_mut(&node_id) {
                node.process(&input_buffers, &output_buffers, playhead, playback_ctx);
            }
        }

        // Get the pointer to the input buffer of the output node
        let Some(input_buffers) = self.get_input_ptr(&self.output_id) else {
            return;
        };
        let Some(output_node) = self.nodes.get_mut(&self.output_id) else {
            return;
        };
        // Process the output node
        // Output data will be written to the output pointer
        output_node.process(&input_buffers, outputs, playhead, playback_ctx);
    }

    fn get_output_ptr(&self, from: &NodeID) -> Option<Vec<*mut u8>> {
        self.node_outputs.get(from).cloned()
    }

    fn get_input_ptr(&self, to: &NodeID) -> Option<Vec<*const u8>> {
        self.node_inputs.get(to).cloned()
    }
}

unsafe impl Send for Graph {}
