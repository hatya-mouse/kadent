use crate::core::audio_engine::{
    MAX_CHANNELS, data_types::PlaybackContext, track::audio_track::AudioTrack,
};

impl AudioTrack {
    // --- LOCAL BUFFER ---

    pub(super) fn init_local_buffers(&mut self, playback_ctx: &PlaybackContext) {
        let buffer_len = playback_ctx.buffer_size * MAX_CHANNELS;
        // Allocate local buffer using MAX_CHANNELS to ensure that the buffer can be reinterpreted as
        // an array of `Sample` type, which has `MAX_CHANNELS` channels
        self.local_buffer = vec![0.0; buffer_len];
        // Also allocate the graph input buffer with the same size
        self.graph_input_buffer = vec![0.0; buffer_len];
    }
}
