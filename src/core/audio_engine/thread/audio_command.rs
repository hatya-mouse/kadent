use std::fmt::Debug;

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
    SetDefaultCtx(PlaybackContext),
    DisarmTrack,
}

impl Debug for AudioCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioCommand::Play => write!(f, "Play"),
            AudioCommand::Pause => write!(f, "Pause"),
            AudioCommand::Seek(pos) => write!(f, "Seek({:?})", pos),
            AudioCommand::UpdateProject(_) => write!(f, "UpdateProject(Project)"),
            AudioCommand::ExportAudio(_, playback_ctx) => {
                write!(f, "ExportAudio(Project, {:?})", playback_ctx)
            }
            AudioCommand::ArmTrack(track_id) => write!(f, "ArmTrack({:?})", track_id),
            AudioCommand::SetOutputDevice(_) => write!(f, "SetOutputDevice(Debug)"),
            AudioCommand::SetDefaultCtx(playback_ctx) => {
                write!(f, "SetDefaultCtx({:?})", playback_ctx)
            }
            AudioCommand::DisarmTrack => write!(f, "DisarmTrack"),
        }
    }
}

#[derive(Clone)]
pub(crate) enum AudioResult {
    ExportedAudio(Vec<f32>, PlaybackContext),
}

#[derive(Debug)]
pub(crate) enum AudioError {
    /// The track preparation failed for a specific track because of an error in the node graph.
    TrackPrepareFailed(TrackID, TrackError),
    /// A thread could not be spawned to an OS error.
    ThreadSpawnFailed(String),
    /// CPAL stream error has occured during playback.
    PlayStreamError(cpal::Error),
    /// CPAL stream error has occured during stream building.
    BuildStreamError(cpal::Error),
    /// An audio command failed, which means that it is likely that the audio thread is frozen or crashed.
    CommandFailed(AudioCommand),
}

unsafe impl Sync for AudioError {}
