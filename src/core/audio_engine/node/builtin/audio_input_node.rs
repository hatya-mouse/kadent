use crate::core::audio_engine::{
    data_types::{PlaybackContext, Sample, TypeInfo},
    graph::error::NodeError,
    node::Node,
    timing::TempoMap,
};
use std::ptr::copy_nonoverlapping;

/// An empty node that just writes the `process` input to the node output.
#[derive(Default, Clone)]
pub(crate) struct AudioInputNode {
    data_type: TypeInfo,
}

impl Node for AudioInputNode {
    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn get_input_names(&self) -> Vec<String> {
        Vec::new()
    }

    fn get_output_names(&self) -> Vec<String> {
        vec!["audio".to_string()]
    }

    fn get_input_len(&self) -> usize {
        0
    }

    fn get_output_len(&self) -> usize {
        1
    }

    fn get_input_type(&self, _index: usize) -> Option<&TypeInfo> {
        None
    }

    fn get_output_type(&self, index: usize) -> Option<&TypeInfo> {
        if index == 0 {
            Some(&self.data_type)
        } else {
            None
        }
    }

    fn update_type_info(&mut self) {
        self.data_type = TypeInfo::new(size_of::<Sample>(), align_of::<Sample>());
    }

    fn prepare(
        &mut self,
        _tempo_map: &TempoMap,
        _playback_ctx: &PlaybackContext,
    ) -> Result<(), Box<dyn NodeError>> {
        self.data_type = TypeInfo::new(size_of::<Sample>(), align_of::<Sample>());
        Ok(())
    }

    fn process(
        &mut self,
        inputs: &[*const u8],
        outputs: &[*mut u8],
        _playhead: usize,
        playback_ctx: &PlaybackContext,
    ) {
        for (input, output) in inputs.iter().zip(outputs.iter()) {
            unsafe {
                // Copy the entire input to the output
                copy_nonoverlapping(
                    *input,
                    *output,
                    self.data_type.actual_size(playback_ctx.buffer_size),
                );
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
