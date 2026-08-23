mod process;
mod render_worker;

pub(super) use render_worker::TrackSyncState;

use crate::core::audio_engine::{
    MAX_CHANNELS,
    audio_data::AudioFilePool,
    data_types::{AudioContext, PlaybackContext},
    graph::Graph,
    timing::{TempoMap, TimeBounds},
    track::{
        RegionID, Track,
        audio_track::{AudioTrack, track_impl::render_worker::spawn_render_worker},
        error::TrackError,
    },
};
use ringbuf::traits::{Consumer, Split};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

impl Track for AudioTrack {
    // --- CLONING ---

    fn clone_box(&self) -> Box<dyn Track> {
        Box::new(self.clone())
    }

    // --- GRAPH GETTING ---

    fn get_graph(&self) -> &Graph {
        &self.graph
    }

    fn get_graph_mut(&mut self) -> &mut Graph {
        &mut self.graph
    }

    // --- GRAPH UPDATING ---

    fn set_graph(&mut self, graph: Graph) {
        self.graph = graph;
    }

    // --- REGION BOUNDS GETTING ---

    fn get_region_bounds(&self, region_id: &RegionID) -> Option<&TimeBounds> {
        self.regions.get(region_id).map(|region| &region.bounds)
    }

    // --- REGION MODIFICATION ---

    fn set_region_bounds(&mut self, region_id: &RegionID, new_bounds: TimeBounds) {
        if let Some(region) = self.regions.get_mut(region_id) {
            region.bounds = new_bounds;
        }
    }

    fn remove_region(&mut self, region_id: &RegionID) {
        self.regions.remove(region_id);
    }

    // --- SEEKING ---

    fn seek(&mut self, playhead: usize, _playback_ctx: &PlaybackContext) {
        if let Some(sync_state) = &self.sync_state {
            sync_state.request_seek(playhead);
        }
    }

    // --- TRACK PROCESSING ---

    fn prepare(
        &mut self,
        audio_pool: &mut AudioFilePool,
        tempo_map: &TempoMap,
        audio_ctx: &AudioContext,
        playback_ctx: &PlaybackContext,
    ) -> Result<(), TrackError> {
        // Prepare the regions
        for region in self.regions.values_mut() {
            region.prepare(audio_pool, tempo_map, audio_ctx, playback_ctx);
        }

        // Stop the old render worker by setting the is_running flag to false
        if let Some(should_worker_stop) = &self.should_worker_stop {
            should_worker_stop.store(true, Ordering::SeqCst);
        }

        // Create a new ring buffer and is_running flag for the new render worker
        let ringbuf_size = playback_ctx.buffer_size * MAX_CHANNELS * 2;
        let (prod, cons) = ringbuf::HeapRb::<f32>::new(ringbuf_size).split();
        self.ringbuf_cons = Some(cons);

        let should_worker_stop = Arc::new(AtomicBool::new(true));
        should_worker_stop.store(false, Ordering::SeqCst);
        self.should_worker_stop = Some(should_worker_stop.clone());

        // Create a sync state to share the playback position between threads
        let sync_state = TrackSyncState::new();
        self.sync_state = Some(sync_state.clone());

        // Spawn a new render worker thread
        spawn_render_worker(
            prod,
            self.regions.values().cloned().collect(),
            tempo_map.clone(),
            playback_ctx.clone(),
            should_worker_stop,
            sync_state,
            0,
        )
        .map_err(|e| TrackError::ThreadSpawnFailed(e.to_string()))?;
        self.is_first_process = true;

        // Initialize the local buffers
        self.init_local_buffers(playback_ctx);
        // Then prepare the graph
        self.graph
            .prepare(tempo_map, playback_ctx)
            .map_err(TrackError::GraphError)
    }

    fn process_to_local_buffer(
        &mut self,
        is_playing: bool,
        playhead: usize,
        _tempo_map: &TempoMap,
        playback_ctx: &PlaybackContext,
    ) {
        if self.is_first_process
            && let Some(sync_state) = &self.sync_state
        {
            sync_state.request_seek(playhead);
            self.is_first_process = false;
        }

        if is_playing {
            let buffer_len = MAX_CHANNELS * playback_ctx.buffer_size;
            if let Some(ringbuf_cons) = &mut self.ringbuf_cons {
                // Pop the rendered audio from the ring buffer into the local buffer
                let popped = ringbuf_cons.pop_slice(&mut self.graph_input_buffer[..buffer_len]);

                // Fill the rest of the local buffer with zeros if the ring buffer didn't have enough data
                if popped < buffer_len {
                    self.graph_input_buffer[popped..buffer_len].fill(0.0);
                }
            } else {
                // If the ring buffer consumer is not available, fill the local buffer with zeros
                self.graph_input_buffer.fill(0.0);
            }
        } else {
            // Mixer::process adds local_buffer into the output every callback regardless of
            // is_playing, so it must be cleared here to prevent the previous buffer from being played repeatedly
            self.graph_input_buffer.fill(0.0);
        }

        let input_ptr = self.graph_input_buffer.as_ptr() as *const u8;
        let output_ptr = self.local_buffer.as_mut_ptr() as *mut u8;

        self.graph
            .process(&[input_ptr], &[output_ptr], playhead, playback_ctx);
    }

    fn get_local_buffer(&self) -> &[f32] {
        &self.local_buffer
    }

    // --- ANY CASTING ---

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
