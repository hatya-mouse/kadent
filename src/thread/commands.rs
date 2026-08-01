use crate::core::{metadata::ProjectMeta, project_ctx::EditorContext};
use kadent_engine::{data_types::AudioContext, mixer::Project};
use std::path::PathBuf;

pub(crate) enum BackgroundThreadCommand {
    SaveProject {
        path: PathBuf,
        project: Project,
        proj_meta: ProjectMeta,
        code_buffers: Vec<(PathBuf, String)>,
    },
    OpenProject {
        path: PathBuf,
    },
    WriteWav {
        path: PathBuf,
        samples: Vec<f32>,
        audio_ctx: AudioContext,
    },
}

pub(crate) enum BackgroundThreadResult {
    SavedProject(std::io::Result<()>),
    OpenedProject(Option<EditorContext>),
    WroteWav(hound::Result<()>),
}
