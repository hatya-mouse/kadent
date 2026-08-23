use crate::core::audio_engine::{
    data_types::MidiEvent,
    mixer::{Mixer, TrackID},
    thread::{AudioCommand, AudioError, AudioResult},
    track::note_track::NoteTrack,
};
use cpal::traits::DeviceTrait;
use ringbuf::{
    HeapCons,
    traits::{Consumer, Producer},
};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering},
    mpsc,
};

#[derive(Clone)]
pub(super) struct OutputCallbackState {
    pub(super) playhead: Arc<AtomicU64>,
    pub(super) playhead_tick: Arc<AtomicI64>,
    pub(super) is_playing: Arc<AtomicBool>,
}

pub(super) struct OutputCallbackContext {
    pub(super) command_cons: HeapCons<AudioCommand>,
    pub(super) midi_cons: ringbuf::HeapCons<MidiEvent>,
    pub(super) vu_prod: ringbuf::HeapProd<f32>,
    pub(super) result_tx: mpsc::Sender<Result<AudioResult, AudioError>>,
}

pub(super) fn output_callback(
    ctx: Arc<Mutex<OutputCallbackContext>>,
    device: cpal::Device,
    config: cpal::StreamConfig,
    state: OutputCallbackState,
    latest_mixer: Arc<Mutex<Option<Mixer>>>,
) -> cpal::Stream {
    let mut armed_track: Option<TrackID> = None;
    let ctx_clone = ctx.clone();

    device
        .build_output_stream(
            config,
            move |data: &mut [f32], _| {
                let Ok(mut ctx) = ctx.try_lock() else {
                    return;
                };
                let Ok(mut guard) = latest_mixer.try_lock() else {
                    return;
                };
                let Some(mixer) = guard.as_mut() else {
                    return;
                };

                let mut current_playhead = state.playhead.load(Ordering::Relaxed);

                // Process all pending commands from the audio command ringbuf
                while let Some(command) = ctx.command_cons.try_pop() {
                    match command {
                        AudioCommand::Seek(seek_position) => {
                            let playhead_sample = seek_position.to_sample(
                                &mixer.project.tempo_map,
                                mixer.playback_ctx.sample_rate,
                            );
                            current_playhead = playhead_sample as u64;
                            state
                                .playhead
                                .store(playhead_sample as u64, Ordering::Relaxed);

                            let playhead_tick = seek_position.to_ticks(&mixer.project.tempo_map);
                            state
                                .playhead_tick
                                .store(playhead_tick.0, Ordering::Relaxed);

                            mixer.seek(playhead_sample);
                        }
                        AudioCommand::ArmTrack(track_id) => {
                            armed_track = Some(track_id);
                        }
                        AudioCommand::DisarmTrack => {
                            armed_track = None;
                        }
                        _ => {}
                    }
                }

                // Drain MIDI events and pass them to the armed NoteTrack
                let midi_events: Vec<MidiEvent> = ctx.midi_cons.pop_iter().collect();
                if !midi_events.is_empty()
                    && let Some(track_id) = armed_track
                    && let Some(track) = mixer.project.tracks.get_mut(&track_id)
                    && let Some(note_track) = track.as_any_mut().downcast_mut::<NoteTrack>()
                {
                    note_track.pass_midi(&midi_events);
                }

                let is_playing = state.is_playing.load(Ordering::Relaxed);

                // Process the audio and fill the output buffer
                mixer.process(is_playing, current_playhead as usize, data);

                // Send the generated waveform data to the main thread for visualization
                let channels = mixer.playback_ctx.channels;
                for ch in 0..channels {
                    let rms = (data
                        .iter()
                        .step_by(channels)
                        .skip(ch)
                        .map(|x| x * x)
                        .sum::<f32>()
                        / (data.len() / channels) as f32)
                        .sqrt();
                    ctx.vu_prod.try_push(rms).ok();
                }

                if is_playing {
                    let new_playhead =
                        current_playhead.saturating_add(mixer.playback_ctx.buffer_size as u64);
                    state.playhead.store(new_playhead, Ordering::Relaxed);
                    state.playhead_tick.store(
                        mixer
                            .project
                            .tempo_map
                            .samples_to_ticks(new_playhead as usize, mixer.playback_ctx.sample_rate)
                            .0,
                        Ordering::Relaxed,
                    );
                }
            },
            move |err| {
                if let Ok(ctx) = ctx_clone.lock() {
                    let _ = ctx.result_tx.send(Err(AudioError::PlayStreamError(err)));
                }
            },
            None,
        )
        .expect("Failed to create a new stream")
}
