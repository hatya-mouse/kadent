use crate::core::audio_engine::{
    audio_data::AudioFilePool,
    data_types::PlaybackContext,
    mixer::ProjectData,
    thread::{AudioError, AudioResult},
};
use std::{sync::mpsc, thread};

pub(super) fn spawn_export_thread(
    result_tx: mpsc::Sender<Result<AudioResult, AudioError>>,
    project: ProjectData,
    playback_ctx: PlaybackContext,
) {
    let result_tx_clone = result_tx.clone();
    let result = thread::Builder::new()
        .name("Export Thread".to_string())
        .spawn(move || {
            let (start_sample, end_sample) = project
                .export_range
                .sample_range(&project.tempo_map, playback_ctx.sample_rate);
            let buffer_size = playback_ctx.buffer_size;
            let channels = playback_ctx.channels;

            // Don't abort the export even if some track fails to prepare
            let mut audio_pool = AudioFilePool::default();
            let (mut mixer, errors) = project.prepare(&mut audio_pool, playback_ctx.clone());
            for (track_id, err) in errors {
                result_tx
                    .send(Err(AudioError::TrackPrepareFailed(track_id, err)))
                    .ok();
            }

            mixer.seek(start_sample);

            let total_samples = (end_sample - start_sample) * channels;
            let mut output: Vec<f32> = Vec::with_capacity(total_samples);
            let mut buf = vec![0.0f32; buffer_size * channels];
            let mut playhead = start_sample;

            while playhead < end_sample {
                mixer.process(true, true, playhead, &mut buf);
                let frames = buffer_size.min(end_sample - playhead);
                output.extend_from_slice(&buf[..frames * channels]);
                playhead += frames;
            }

            result_tx
                .send(Ok(AudioResult::ExportedAudio(output, playback_ctx)))
                .unwrap();
        });

    match result {
        Ok(_) => (),
        Err(e) => {
            result_tx_clone
                .send(Err(AudioError::ThreadSpawnFailed(e.to_string())))
                .ok();
        }
    }
}
