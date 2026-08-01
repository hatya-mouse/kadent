//! An implementation of the background thread that processes heavy tasks such as file I/O operations.

mod audio_import;
mod commands;
mod project;

pub(crate) use commands::{
    BackgroundTaskStatus, BackgroundThreadCommand, BackgroundThreadResult, DecodedAudio,
};

use crate::{
    background_thread::{
        audio_import::run_decode_wav,
        project::{run_save_project, run_write_wav},
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
    for command in command_rx {
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
                audio_ctx,
            } => BackgroundThreadResult::WroteWav(run_write_wav(&path, &samples, &audio_ctx)),
            BackgroundThreadCommand::ImportAudio {
                track_id,
                start,
                path,
            } => {
                let result = run_decode_wav(&path);
                BackgroundThreadResult::ImportedAudio {
                    track_id,
                    start,
                    result,
                }
            }
        };
        result_tx.send(result).ok();
    }
}
