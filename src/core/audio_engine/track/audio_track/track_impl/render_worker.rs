use crate::core::audio_engine::{
    MAX_CHANNELS, THREAD_WAIT_DURATION,
    data_types::PlaybackContext,
    timing::TempoMap,
    track::{audio_track::AudioRegion, error::TrackError},
};
use ringbuf::traits::{Observer, Producer};
use std::sync::{
    Arc, Condvar, Mutex,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

/// A struct used to stop the render worker thread.
pub(in crate::core::audio_engine::track::audio_track) struct RenderWorker {
    pub(crate) should_stop: Arc<AtomicBool>,
    pub(crate) handle: std::thread::JoinHandle<()>,
}

impl RenderWorker {
    pub(crate) fn signal_stop(&self) {
        self.should_stop.store(true, Ordering::Relaxed);
    }

    pub(crate) fn join_thread(self) -> Result<(), TrackError> {
        self.signal_stop();
        self.handle
            .join()
            .map_err(|_| TrackError::RenderWorkerPanicked)
    }
}

/// A struct used to synchronize the playhead between the main thread and the render worker thread when seeking.
#[derive(Clone)]
pub(in crate::core::audio_engine::track::audio_track) struct TrackSyncState {
    seek_requested: Arc<AtomicBool>,
    seek_target: Arc<AtomicUsize>,
}

impl TrackSyncState {
    pub(crate) fn new() -> Self {
        Self {
            seek_requested: Arc::new(AtomicBool::new(false)),
            seek_target: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub(crate) fn request_seek(&self, target: usize) {
        self.seek_requested.store(true, Ordering::Release);
        self.seek_target.store(target, Ordering::Release);
    }

    pub(crate) fn consume_seek(&self) -> Option<usize> {
        if self.seek_requested.swap(false, Ordering::AcqRel) {
            Some(self.seek_target.load(Ordering::Acquire))
        } else {
            None
        }
    }
}

/// A struct used for the exporting thread to wait until the render worker has finished rendering all the audio data.
pub(in crate::core::audio_engine::track::audio_track) struct ExportWaitState {
    pub(super) state: Mutex<()>,
    pub(super) condvar: Condvar,
}

impl ExportWaitState {
    pub(super) fn new() -> Self {
        Self {
            state: Mutex::new(()),
            condvar: Condvar::new(),
        }
    }
}

pub(super) fn spawn_render_worker(
    mut producer: ringbuf::HeapProd<f32>,
    mut regions: Vec<AudioRegion>,
    tempo_map: TempoMap,
    playback_ctx: PlaybackContext,
    should_stop: Arc<AtomicBool>,
    sync_state: TrackSyncState,
    export_wait_state: Arc<ExportWaitState>,
) -> std::io::Result<std::thread::JoinHandle<()>> {
    let mut worker_playhead = 0usize;

    std::thread::Builder::new()
        .name("Audio Track Render Worker".to_string())
        .spawn(move || {
            let buffer_len = MAX_CHANNELS * playback_ctx.buffer_size;
            let mut render_buf = vec![0.0; buffer_len];

            // Keep render worker running until the is_running flag is set to false
            while !should_stop.load(Ordering::Relaxed) {
                // Synchronize the worker playhead if a seek has been requested
                if let Some(new_playhead) = sync_state.consume_seek() {
                    worker_playhead = new_playhead;
                }

                if producer.vacant_len() >= buffer_len {
                    // Clear the render buffer before rendering
                    render_buf.fill(0.0);
                    // Render each region into the render buffer
                    for region in regions.iter_mut() {
                        region.render_buffer(
                            worker_playhead,
                            &mut render_buf,
                            &tempo_map,
                            &playback_ctx,
                        );
                    }

                    if let Ok(_guard) = export_wait_state.state.lock() {
                        // Push the rendered buffer into the ring buffer
                        producer.push_slice(&render_buf);
                        // Notify the export thread that new data is available
                        export_wait_state.condvar.notify_one();
                    }

                    // Advance the worker playhead by the buffer size
                    worker_playhead += playback_ctx.buffer_size;
                } else {
                    std::thread::sleep(THREAD_WAIT_DURATION);
                }
            }
        })
}
