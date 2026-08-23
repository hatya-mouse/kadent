use crate::core::audio_engine::{
    audio_data::AudioFilePool,
    data_types::PlaybackContext,
    mixer::{Mixer, ProjectData},
    thread::{AudioError, AudioResult},
};
use std::sync::{Arc, Condvar, Mutex, mpsc};

pub(crate) enum PreparationThreadRequest {
    PrepareProject(Box<ProjectData>, PlaybackContext),
    Terminate,
}

pub(crate) struct PrepareState {
    pub(crate) request: Mutex<Option<PreparationThreadRequest>>,
    pub(crate) condvar: Condvar,
}

impl PrepareState {
    pub(crate) fn new() -> Self {
        PrepareState {
            request: Mutex::new(None),
            condvar: Condvar::new(),
        }
    }

    pub(crate) fn request_preparation(
        &self,
        project: Box<ProjectData>,
        playback_ctx: PlaybackContext,
    ) {
        let mut guard = self.request.lock().unwrap();
        *guard = Some(PreparationThreadRequest::PrepareProject(
            project,
            playback_ctx,
        ));
        self.condvar.notify_one();
    }

    pub(crate) fn request_termination(&self) {
        let mut guard = self.request.lock().unwrap();
        *guard = Some(PreparationThreadRequest::Terminate);
        self.condvar.notify_one();
    }
}

pub(super) fn spawn_preparation_thread(
    state: Arc<PrepareState>,
    result_tx: mpsc::Sender<Result<AudioResult, AudioError>>,
    mixer_arc: Arc<Mutex<Option<Mixer>>>,
) {
    let mut audio_pool = AudioFilePool::default();

    let result_tx_clone = result_tx.clone();
    let result = std::thread::Builder::new()
        .name("Project Preparation Thread".to_string())
        .spawn(move || {
            loop {
                let req = {
                    let Ok(mut guard) = state.request.lock() else {
                        break;
                    };

                    // Wait for a new project to prepare or for termination signal
                    while guard.is_none() {
                        guard = match state.condvar.wait(guard) {
                            Ok(g) => g,
                            // Terminate the thread if the lock is poisoned
                            Err(_) => return,
                        }
                    }
                    guard.take().expect("Request should be Some at this point")
                };

                match req {
                    PreparationThreadRequest::PrepareProject(new_project, playback_ctx) => {
                        let (prepared_mixer, errors) =
                            new_project.prepare(&mut audio_pool, playback_ctx);
                        for (track_id, graph_error) in errors {
                            let _ = result_tx
                                .send(Err(AudioError::TrackPrepareFailed(track_id, graph_error)));
                        }

                        if let Ok(mut mixer_guard) = mixer_arc.lock() {
                            mixer_guard.replace(prepared_mixer);
                        }
                    }
                    PreparationThreadRequest::Terminate => {
                        // Break the loop if a termination request is received
                        return;
                    }
                }
            }
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
