use crate::core::audio_engine::{
    THREAD_WAIT_DURATION,
    data_types::{MidiEvent, PlaybackContext},
    mixer::Mixer,
    thread::{
        AudioCommand, AudioError, AudioResult, export,
        output_callback::{OutputCallbackContext, OutputCallbackState, output_callback},
        preparation_thread::{PrepareState, spawn_preparation_thread},
    },
};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::traits::{Consumer, Producer, Split};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering},
    mpsc,
};

struct OutputState {
    device: cpal::Device,
    config: cpal::StreamConfig,
    stream: Option<cpal::Stream>,
}

pub(super) fn audio_thread(
    command_rx: mpsc::Receiver<AudioCommand>,
    result_tx: mpsc::Sender<Result<AudioResult, AudioError>>,
    mut midi_cons: ringbuf::HeapCons<MidiEvent>,
    vu_prod: ringbuf::HeapProd<f32>,
    playhead: Arc<AtomicU64>,
    playhead_tick: Arc<AtomicI64>,
    mut default_ctx: PlaybackContext,
) {
    let (mut command_prod, command_cons) = ringbuf::HeapRb::<AudioCommand>::new(64).split();
    let (mut midi_sub_prod, midi_sub_cons) = ringbuf::HeapRb::<MidiEvent>::new(64).split();

    // A variable to hold the latest prepared mixer, which will be used by the output callback
    let latest_mixer = Arc::new(Mutex::new(None));
    // Manage is_playing using Arc
    let is_playing = Arc::new(AtomicBool::new(false));
    // The state to notify the preparation thread to prepare a new project
    let prepare_state = Arc::new(PrepareState::new());

    // Spawn a project preparation thread
    spawn_preparation_thread(
        prepare_state.clone(),
        result_tx.clone(),
        latest_mixer.clone(),
    );

    // Get a cpal device
    let host = cpal::default_host();
    let initial_device = host
        .default_output_device()
        .expect("Expect a default output device");

    // Create an output callback
    let callback_ctx = Arc::new(Mutex::new(OutputCallbackContext {
        command_cons,
        midi_cons: midi_sub_cons,
        vu_prod,
        result_tx: result_tx.clone(),
    }));
    let callback_state = OutputCallbackState {
        playhead,
        playhead_tick,
        is_playing: is_playing.clone(),
    };
    let mut output_state = OutputState {
        config: resolve_output_config(&initial_device, &default_ctx),
        device: initial_device,
        stream: None,
    };
    output_state.stream = output_callback(
        callback_ctx.clone(),
        output_state.device.clone(),
        output_state.config,
        callback_state.clone(),
        latest_mixer.clone(),
    );

    if let Some(stream) = output_state.stream.as_ref()
        && let Err(err) = stream.play()
    {
        result_tx
            .send(Err(AudioError::PlayStreamError(err)))
            .unwrap();
    }

    // Create a message loop
    'message_loop: loop {
        loop {
            // If the receiver thread is disconnected, break the loop and exit the thread
            let command = match command_rx.try_recv() {
                Ok(command) => command,
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => break 'message_loop,
            };

            match command {
                AudioCommand::Play => {
                    is_playing.store(true, Ordering::Release);
                }
                AudioCommand::Pause => {
                    is_playing.store(false, Ordering::Release);
                }
                AudioCommand::UpdateProject(new_project) => {
                    // The latest playback context, tied to the latest prepared mixer.
                    let output_playback_ctx = PlaybackContext::from_stream_config(
                        &output_state.config,
                        default_ctx.buffer_size,
                    );
                    prepare_state.request_preparation(new_project, output_playback_ctx);
                }
                AudioCommand::ExportAudio(project, playback_ctx) => {
                    let result_tx = result_tx.clone();
                    export::spawn_export_thread(result_tx, *project, playback_ctx);
                }
                AudioCommand::SetOutputDevice(device) => {
                    output_state.device = device;
                    recreate_output_callback(
                        &mut output_state,
                        &callback_ctx,
                        callback_state.clone(),
                        &mut midi_sub_prod,
                        &latest_mixer,
                        &mut default_ctx,
                    );
                    reprepare_mixer(
                        latest_mixer.clone(),
                        prepare_state.clone(),
                        &output_state,
                        &default_ctx,
                    );

                    if let Some(stream) = output_state.stream.as_ref()
                        && let Err(err) = stream.play()
                    {
                        result_tx
                            .send(Err(AudioError::PlayStreamError(err)))
                            .unwrap();
                    }
                }
                AudioCommand::SetDefaultCtx(new_default_ctx) => {
                    default_ctx = new_default_ctx;
                    recreate_output_callback(
                        &mut output_state,
                        &callback_ctx,
                        callback_state.clone(),
                        &mut midi_sub_prod,
                        &latest_mixer,
                        &mut default_ctx,
                    );
                    reprepare_mixer(
                        latest_mixer.clone(),
                        prepare_state.clone(),
                        &output_state,
                        &default_ctx,
                    );

                    if let Some(stream) = output_state.stream.as_ref()
                        && let Err(err) = stream.play()
                    {
                        result_tx
                            .send(Err(AudioError::PlayStreamError(err)))
                            .unwrap();
                    }
                }
                AudioCommand::Seek(_) | AudioCommand::ArmTrack(_) | AudioCommand::DisarmTrack => {
                    if let Err(command) = command_prod.try_push(command) {
                        result_tx
                            .send(Err(AudioError::CommandFailed(command)))
                            .unwrap();
                    }
                }
            }
        }

        // Send the MIDI events from the midi_cons to the midi_sub_prod
        while let Some(midi_event) = midi_cons.try_pop() {
            midi_sub_prod.try_push(midi_event).ok();
        }

        std::thread::sleep(THREAD_WAIT_DURATION);
    }

    // Stream will be dropped here and the output callback should stop
    // Terminate preparation thread
    prepare_state.request_termination();
}

fn recreate_output_callback(
    output_state: &mut OutputState,
    callback_ctx: &Arc<Mutex<OutputCallbackContext>>,
    callback_state: OutputCallbackState,
    midi_sub_prod: &mut ringbuf::HeapProd<MidiEvent>,
    latest_mixer: &Arc<Mutex<Option<Mixer>>>,
    default_ctx: &mut PlaybackContext,
) {
    output_state.stream.take();

    // Create a new MIDI ring buffer and split it into producer and consumer
    let (new_sub_prod, new_sub_cons) = ringbuf::HeapRb::<MidiEvent>::new(64).split();
    *midi_sub_prod = new_sub_prod;

    callback_ctx.lock().unwrap().midi_cons = new_sub_cons;

    output_state.config = resolve_output_config(&output_state.device, default_ctx);
    // Then get the latest mixer to pass to the new output callback
    output_state.stream = output_callback(
        callback_ctx.clone(),
        output_state.device.clone(),
        output_state.config,
        callback_state.clone(),
        latest_mixer.clone(),
    );
}

/// Prepares the project in the existing mixer and updates the playback context.
fn reprepare_mixer(
    latest_mixer: Arc<Mutex<Option<Mixer>>>,
    prepare_state: Arc<PrepareState>,
    output_state: &OutputState,
    default_ctx: &PlaybackContext,
) {
    if let Ok(mut guard) = latest_mixer.lock()
        && let Some(mixer) = guard.take()
    {
        let project = mixer.take_project();
        let output_playback_ctx =
            PlaybackContext::from_stream_config(&output_state.config, default_ctx.buffer_size);
        prepare_state.request_preparation(Box::new(project), output_playback_ctx);
    }
}

/// Resolves the output configuration for the given device, falling back to the provided playback context if necessary.
fn resolve_output_config(device: &cpal::Device, fallback: &PlaybackContext) -> cpal::StreamConfig {
    device
        .default_output_config()
        .map(|config| config.config())
        .unwrap_or_else(|_| cpal::StreamConfig {
            channels: fallback.channels as u16,
            sample_rate: fallback.sample_rate as u32,
            buffer_size: cpal::BufferSize::Fixed(fallback.buffer_size as u32),
        })
}
