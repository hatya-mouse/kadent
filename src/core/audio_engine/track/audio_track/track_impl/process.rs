use crate::core::audio_engine::{
    MAX_CHANNELS, data_types::PlaybackContext, track::audio_track::AudioTrack,
};
use ringbuf::traits::Observer;

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

    // --- RENDER THREAD WAITING ---

    pub(super) fn wait_for_rendered_samples(&self, buffer_len: usize) {
        if let Some(export_wait_state) = &self.export_wait_state
            && let Some(ringbuf_cons) = &self.ringbuf_cons
        {
            let mut guard = export_wait_state.state.lock().unwrap();

            // Wait until the ring buffer has enough samples to fill the local buffer
            while ringbuf_cons.occupied_len() < buffer_len {
                guard = export_wait_state.condvar.wait(guard).unwrap();
            }
        }
    }
}
