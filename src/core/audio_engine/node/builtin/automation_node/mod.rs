mod constant;
mod curve;
mod float;
mod keyframe;
mod track;

pub(crate) use curve::CurveType;
pub(crate) use keyframe::{Keyframe, NormalizedKeyframe};
pub(crate) use track::{AutomationTrack, AutomationTrackType};

use crate::core::audio_engine::{
    data_types::{PlaybackContext, TypeInfo},
    graph::error::NodeError,
    node::Node,
    timing::TempoMap,
};
use std::slice;

#[derive(Clone)]
pub(crate) struct AutomationNode {
    /// The automation track that stores keyframes.
    pub(crate) track: AutomationTrack,
    /// The cached data type of the output value.
    output_type: TypeInfo,
}

impl AutomationNode {
    pub(crate) fn new(track: AutomationTrack) -> Self {
        let mut node = Self {
            track,
            output_type: TypeInfo::default(),
        };
        node.update_type_info();
        node
    }
}

impl Node for AutomationNode {
    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn get_input_names(&self) -> Vec<String> {
        Vec::new()
    }

    fn get_output_names(&self) -> Vec<String> {
        vec!["value".to_string()]
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
            Some(&self.output_type)
        } else {
            None
        }
    }

    fn update_type_info(&mut self) {
        self.output_type = match self.track {
            AutomationTrack::Float { .. } => TypeInfo::new(4, 4),
            AutomationTrack::Int { .. } => TypeInfo::new(4, 4),
            AutomationTrack::Bool { .. } => TypeInfo::new(1, 1),
        };
    }

    fn prepare(
        &mut self,
        tempo_map: &TempoMap,
        playback_ctx: &PlaybackContext,
    ) -> Result<(), Box<dyn NodeError>> {
        self.track.prepare(tempo_map, playback_ctx);
        Ok(())
    }

    fn process(
        &mut self,
        _inputs: &[*const u8],
        outputs: &[*mut u8],
        playhead: usize,
        playback_ctx: &PlaybackContext,
    ) {
        if let Some(&output) = outputs.first() {
            let data_len = self.output_type.actual_size(playback_ctx.buffer_size);
            let slice: &mut [u8] = unsafe { slice::from_raw_parts_mut(output, data_len) };
            self.track.process(slice, playhead, playback_ctx);
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
