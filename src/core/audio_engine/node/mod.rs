pub(crate) mod builtin;

use crate::core::audio_engine::{
    data_types::{PlaybackContext, TypeInfo},
    graph::error::NodeError,
    timing::TempoMap,
};
use std::{any::Any, fmt::Debug};

pub(crate) trait Node: Send + Any + Debug {
    /// Clones the node.
    fn clone_box(&self) -> Box<dyn Node>;

    /// Returns a vector of the names of all inputs.
    fn get_input_names(&self) -> Vec<String>;

    /// Returns a vector of the names of all outputs.
    fn get_output_names(&self) -> Vec<String>;

    /// Returns the number of outputs.
    fn get_output_len(&self) -> usize;

    /// Returns the number of inputs.
    fn get_input_len(&self) -> usize;

    /// Returns the value type information of the specified input.
    fn get_input_type(&self, index: usize) -> Option<&TypeInfo>;

    /// Returns the value type information of the specified output.
    fn get_output_type(&self, index: usize) -> Option<&TypeInfo>;

    /// Updates the type info of the node with the given playback context.
    fn update_type_info(&mut self);

    /// Prepares the node for processing.
    fn prepare(
        &mut self,
        tempo_map: &TempoMap,
        playback_ctx: &PlaybackContext,
    ) -> Result<(), Box<dyn NodeError>>;

    /// Processes the given input pointer and writes the output to the output pointer.
    fn process(
        &mut self,
        inputs: &[&[u8]],
        outputs: &mut [&mut [u8]],
        playhead: usize,
        playback_ctx: &PlaybackContext,
    );

    /// Converts a reference to the node to any.
    fn as_any(&self) -> &dyn Any;

    /// Converts a mutable reference to the node to any.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
