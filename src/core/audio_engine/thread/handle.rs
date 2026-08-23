use crate::core::audio_engine::thread::{AudioCommand, AudioError, AudioResult};
use std::sync::{Arc, atomic::AtomicI64, mpsc};

/// A struct to communicate with the audio thread.
pub(crate) struct AudioThreadHandle {
    pub(crate) audio_command_tx: mpsc::Sender<AudioCommand>,
    pub(crate) result_rx: mpsc::Receiver<Result<AudioResult, AudioError>>,
    pub(crate) vu_consumer: ringbuf::HeapCons<f32>,
    pub(crate) playhead_tick: Arc<AtomicI64>,
}
