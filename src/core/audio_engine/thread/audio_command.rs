use crate::core::audio_engine::{
    data_types::PlaybackContext,
    mixer::{ProjectData, TrackID},
    timing::TimePosition,
    track::error::TrackError,
};
use cpal::Device;

#[derive(Clone)]
pub(crate) enum AudioCommand {
    Play,
    Pause,
    Seek(TimePosition),
    UpdateProject(Box<ProjectData>),
    ExportAudio(Box<ProjectData>, PlaybackContext),
    ArmTrack(TrackID),
    SetOutputDevice(Device),
    SetPlaybackCtx(PlaybackContext),
    DisarmTrack,
}

#[derive(Clone)]
pub(crate) enum AudioResult {
    ExportedAudio(Vec<f32>),
}

pub(crate) enum AudioError {
    /// The track preparation failed for a specific track because of an error in the node graph.
    TrackPrepareFailed(TrackID, TrackError),
    /// A thread could not be spawned to an OS error.
    ThreadSpawnFailed(String),
    /// CPAL stream error has occured during playback.
    PlayStreamError(cpal::Error),
    /// An audio command failed, which means that it is likely that the audio thread is frozen or crashed.
    CommandFailed(AudioCommand),
}

unsafe impl Sync for AudioError {}
