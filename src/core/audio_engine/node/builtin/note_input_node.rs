use crate::core::audio_engine::{
    data_types::{EventSlot, PlaybackContext, TypeInfo},
    graph::error::NodeError,
    node::Node,
    timing::TempoMap,
};

/// An empty node that just writes the `process` input to the node output.
#[derive(Default, Clone)]
pub(crate) struct NoteInputNode {
    data_type: TypeInfo,
}

impl Node for NoteInputNode {
    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn get_input_names(&self) -> Vec<String> {
        Vec::new()
    }

    fn get_output_names(&self) -> Vec<String> {
        vec!["notes".to_string()]
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
        self.data_type = TypeInfo::new(size_of::<EventSlot>(), 4);
    }

    fn prepare(
        &mut self,
        _tempo_map: &TempoMap,
        _playback_ctx: &PlaybackContext,
    ) -> Result<(), Box<dyn NodeError>> {
        self.data_type = TypeInfo::new(size_of::<EventSlot>(), 4);
        Ok(())
    }

    fn process(
        &mut self,
        inputs: &[&[u8]],
        outputs: &mut [&mut [u8]],
        _playhead: usize,
        _playback_ctx: &PlaybackContext,
    ) {
        for (input, output) in inputs.iter().zip(outputs.iter_mut()) {
            output.copy_from_slice(input);
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
