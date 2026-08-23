mod audio_command;
mod audio_thread;
mod export;
mod handle;
mod output_callback;
mod preparation_thread;

pub(crate) use audio_command::{AudioCommand, AudioError, AudioResult};
pub(crate) use handle::AudioThreadHandle;

use crate::core::audio_engine::data_types::{MidiEvent, PlaybackContext};
use ringbuf::{HeapRb, traits::Split};
use std::{
    sync::{
        Arc,
        atomic::{AtomicI64, AtomicU64},
        mpsc,
    },
    thread,
};

pub(crate) struct AudioThread;

impl AudioThread {
    pub(crate) fn spawn(
        playback_ctx: PlaybackContext,
    ) -> (AudioThreadHandle, ringbuf::HeapProd<MidiEvent>) {
        // MPSC channels to send commands to the processing threads from the host.
        let (audio_command_tx, audio_command_rx) = mpsc::channel();
        // MPSC channel to send the results back to the host.
        let (result_tx, result_rx) = mpsc::channel();
        // Shared playhead position using Arc and AtomicUsize for thread-safe access.
        let playhead = Arc::new(AtomicU64::new(0));
        // Shared playhead tick using Arc and AtomicI64 for thread-safe access.
        let playhead_tick = Arc::new(AtomicI64::new(0));
        // A ringbuf to send MIDI events to the audio thread from the midi thread.
        let (midi_producer, midi_consumer) = HeapRb::<MidiEvent>::new(64).split();
        // A ringbuf to send the calculated VU levels to the host.
        let (vu_producer, vu_consumer) = HeapRb::<f32>::new(playback_ctx.channels * 2).split();

        // --- MAIN AUDIO THREAD ---
        let result_tx_clone = result_tx.clone();
        let playhead_clone = playhead.clone();
        let playhead_tick_clone = playhead_tick.clone();
        let result = thread::Builder::new()
            .name("Audio Thread".to_string())
            .spawn(move || {
                audio_thread::audio_thread(
                    audio_command_rx,
                    result_tx,
                    midi_consumer,
                    vu_producer,
                    playhead_clone,
                    playhead_tick_clone,
                    playback_ctx,
                );
            });
        match result {
            Ok(_) => (),
            Err(e) => {
                result_tx_clone
                    .send(Err(AudioError::ThreadSpawnFailed(e.to_string())))
                    .ok();
            }
        }

        (
            AudioThreadHandle {
                audio_command_tx,
                result_rx,
                vu_consumer,
                playhead_tick,
            },
            midi_producer,
        )
    }
}
