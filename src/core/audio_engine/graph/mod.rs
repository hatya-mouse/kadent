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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) struct InputKey(pub(crate) NodeID, pub(crate) usize);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) struct OutputKey(pub(crate) NodeID, pub(crate) usize);

#[derive(Debug, Default, Clone)]
pub(crate) enum InputSource {
    Edge(OutputKey),
    #[default]
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
    /// Index of outer vector corresponds to the output index of the node.
    output_buffers: HashMap<NodeID, Vec<Vec<u8>>>,
    /// Cached input sources for each node, which are used to get the input buffers for processing.
    node_inputs: HashMap<NodeID, Vec<InputSource>>,
    /// Zero buffer used for nodes that have no input connected.
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
            if let InputSource::Edge(OutputKey(from_node, _)) = source {
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
    pub(crate) fn add_edge(&mut self, from: OutputKey, to: InputKey) -> Result<(), GraphError> {
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

        self.input_sources.insert(to, InputSource::Edge(from));
        Ok(())
    }

    /// Removes the edge from the graph.
    pub(crate) fn remove_edge(&mut self, to: InputKey) {
        self.input_sources.insert(to, InputSource::Zero);
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
                if let InputSource::Edge(OutputKey(from_node, from_output)) = source {
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
        output_buffers: &mut HashMap<NodeID, Vec<Vec<u8>>>,
        playback_ctx: &PlaybackContext,
    ) -> Result<(), GraphError> {
        // Create buffers for all outputs
        let mut buffers = Vec::with_capacity(node.get_output_len());
        for output_index in 0..node.get_output_len() {
            let output_type = node
                .get_output_type(output_index)
                .ok_or(GraphError::OutputTypeUnavailable(*node_id, output_index))?;
            let buffer = vec![0u8; output_type.actual_size(playback_ctx.buffer_size)];

            // Insert the output buffer to the output_buffers
            buffers.push(buffer);
        }
        output_buffers.insert(*node_id, buffers);

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
                    playback_ctx,
                )?;
            }
        }

        // Fill the missing input sources with Zero
        for node in self.nodes.keys() {
            let input_len = self.nodes.get(node).map_or(0, |node| node.get_input_len());
            for input_index in 0..input_len {
                let key = InputKey(*node, input_index);
                self.input_sources.entry(key).or_default();
            }
        }

        // Construct input_sources cache in advance
        self.node_inputs.clear();
        let mut indexed_sources: HashMap<NodeID, Vec<(usize, &InputSource)>> =
            HashMap::with_capacity(self.nodes.len());

        for (to, source) in &self.input_sources {
            indexed_sources
                .entry(to.0)
                .or_default()
                .push((to.1, source));
        }

        // Include all nodes in the node_inputs map, even if they have no input sources
        for to_node in self.nodes.keys() {
            let mut sources = indexed_sources.remove(to_node).unwrap_or_default();
            sources.sort_by_key(|(input_index, _)| *input_index);
            let input_sources: Vec<InputSource> =
                sources.iter().map(|&(_, source)| source.clone()).collect();
            self.node_inputs.insert(*to_node, input_sources);
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

        Ok(())
    }

    /// Processes the graph in the sorted order and writes the result in the output pointer.
    /// The host must pass the audio context which is as the same as the one given in the `set_audio_ctx` function.
    pub(crate) fn process(
        &mut self,
        inputs: &[&[u8]],
        outputs: &mut [&mut [u8]],
        playhead: usize,
        playback_ctx: &PlaybackContext,
    ) {
        // Get the pointer to the output buffer of the input node
        let Some(mut output_buffers) =
            Self::get_output_mut(&self.input_id, &mut self.output_buffers, &self.nodes)
        else {
            return;
        };
        let Some(input_node) = self.nodes.get_mut(&self.input_id) else {
            return;
        };
        // Process the input node
        input_node.process(inputs, &mut output_buffers, playhead, playback_ctx);

        for node_id in &self.sorted_nodes {
            // Capture the output buffers of the node and temporary remove it from the output_buffers map
            let Some(mut current_outputs) = self.output_buffers.remove(node_id) else {
                return;
            };

            // Get the pointer to the input buffer of the node
            let Some(input_buffers) = Self::get_input_ref(
                node_id,
                &self.output_buffers,
                &self.node_inputs,
                &self.zero_buffer,
                &self.nodes,
            ) else {
                // Put the output buffers back to the output_buffers map
                self.output_buffers.insert(*node_id, current_outputs);
                return;
            };

            // Get the pointer to the output buffer of the node
            let mut output_buffers = current_outputs
                .iter_mut()
                .map(|buffer| buffer.as_mut_slice())
                .collect::<Vec<_>>();

            // Pass the pointers and process
            if let Some(node) = self.nodes.get_mut(node_id) {
                node.process(&input_buffers, &mut output_buffers, playhead, playback_ctx);
            }

            // Put the output buffers back to the output_buffers map
            self.output_buffers.insert(*node_id, current_outputs);
        }

        // Get the pointer to the input buffer of the output node
        let Some(input_buffers) = Self::get_input_ref(
            &self.output_id,
            &self.output_buffers,
            &self.node_inputs,
            &self.zero_buffer,
            &self.nodes,
        ) else {
            return;
        };
        let Some(output_node) = self.nodes.get_mut(&self.output_id) else {
            return;
        };
        // Process the output node
        // Output data will be written to the output pointer
        output_node.process(&input_buffers, outputs, playhead, playback_ctx);
    }

    /// Returns mutable references to each output buffer of the node with the given NodeID.
    fn get_output_mut<'a>(
        from: &NodeID,
        output_buffers: &'a mut HashMap<NodeID, Vec<Vec<u8>>>,
        nodes: &HashMap<NodeID, Box<dyn Node>>,
    ) -> Option<Vec<&'a mut [u8]>> {
        let refs: Vec<&mut [u8]> = output_buffers.get_mut(from).map(|buffers| {
            buffers
                .iter_mut()
                .map(|buffer| buffer.as_mut_slice())
                .collect()
        })?;
        if nodes.get(from).map_or(0, |node| node.get_output_len()) != refs.len() {
            // Return None if the number of output buffers does not match the number of outputs of the node
            return None;
        }
        Some(refs)
    }

    /// Returns references to each input buffer of the node with the given NodeID.
    fn get_input_ref<'a>(
        to: &NodeID,
        output_buffers: &'a HashMap<NodeID, Vec<Vec<u8>>>,
        node_inputs: &HashMap<NodeID, Vec<InputSource>>,
        zero_buffer: &'a [u8],
        nodes: &HashMap<NodeID, Box<dyn Node>>,
    ) -> Option<Vec<&'a [u8]>> {
        let input_len = nodes.get(to).map_or(0, |node| node.get_input_len());
        let refs: Vec<&[u8]> = node_inputs.get(to).and_then(|sources| {
            sources
                .iter()
                .map(|source| match source {
                    InputSource::Edge(key) => output_buffers
                        .get(&key.0)
                        .and_then(|b| b.get(key.1))
                        .map(|b| b.as_slice()),
                    InputSource::Zero => Some(zero_buffer),
                })
                .collect::<Option<_>>()
        })?;
        if input_len != refs.len() {
            // Return None if the number of input buffers does not match the number of inputs of the node
            return None;
        }
        Some(refs)
    }
}

unsafe impl Send for Graph {}
