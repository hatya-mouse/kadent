//! An implementation of the background thread that processes heavy tasks such as file I/O operations.

mod audio_import;
mod commands;
mod project;
mod waveform;

pub(crate) use commands::{
    BackgroundTaskStatus, BackgroundThreadCommand, BackgroundThreadResult, DecodedAudio,
    WaveformLod,
};

use crate::{
    background_thread::{
        audio_import::run_decode_wav,
        project::{run_save_project, run_write_wav},
        waveform::run_generate_waveform,
    },
    storage::project::open_project_to_ctx,
};
use std::sync::mpsc;

pub(crate) struct BackgroundThreadHandle {
    pub command_tx: mpsc::Sender<BackgroundThreadCommand>,
    pub result_rx: mpsc::Receiver<BackgroundThreadResult>,
}

pub(crate) fn spawn_background_thread() -> BackgroundThreadHandle {
    let (command_tx, command_rx) = mpsc::channel::<BackgroundThreadCommand>();
    let (result_tx, result_rx) = mpsc::channel::<BackgroundThreadResult>();

    std::thread::spawn(move || {
        background_thread(command_rx, result_tx);
    });

    BackgroundThreadHandle {
        command_tx,
        result_rx,
    }
}

fn background_thread(
    command_rx: mpsc::Receiver<BackgroundThreadCommand>,
    result_tx: mpsc::Sender<BackgroundThreadResult>,
) {
    while let Ok(command) = command_rx.recv() {
        let result = match command {
            BackgroundThreadCommand::SaveProject {
                path,
                project,
                project_meta,
                code_buffers,
            } => BackgroundThreadResult::SavedProject(run_save_project(
                &path,
                &project,
                &project_meta,
                &code_buffers,
            )),
            BackgroundThreadCommand::OpenProject { path } => {
                BackgroundThreadResult::OpenedProject(open_project_to_ctx(path).map(Box::new))
            }
            BackgroundThreadCommand::WriteWav {
                path,
                samples,
                export_ctx,
            } => BackgroundThreadResult::WroteWav(run_write_wav(&path, &samples, &export_ctx)),
            BackgroundThreadCommand::ImportAudio {
                file_name,
                start,
                path,
            } => {
                let result = run_decode_wav(path);
                BackgroundThreadResult::ImportedAudio {
                    file_name,
                    start,
                    result,
                }
            }
            BackgroundThreadCommand::GenerateWaveform {
                track_id,
                region_id,
                source,
                channels,
            } => {
                let waveform = run_generate_waveform(&source, channels);
                BackgroundThreadResult::GeneratedWaveform {
                    track_id,
                    region_id,
                    waveform,
                }
            }
        };
        result_tx.send(result).ok();
    }
}
