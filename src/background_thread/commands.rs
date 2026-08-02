use crate::core::{metadata::ProjectMeta, project_ctx::EditorContext};
use kadent_engine::{data_types::AudioContext, mixer::Project};
use std::path::PathBuf;

pub(crate) struct DecodedAudio {
    pub data: Vec<f32>,
    pub frames: usize,
    pub sample_rate: u32,
    pub channels: u16,
}

pub(crate) enum BackgroundThreadCommand {
    SaveProject {
        path: PathBuf,
        project: Box<Project>,
        project_meta: Box<ProjectMeta>,
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
    ImportAudio {
        file_name: Option<String>,
        path: PathBuf,
    },
}

pub(crate) enum BackgroundThreadResult {
    SavedProject(std::io::Result<()>),
    OpenedProject(Option<Box<EditorContext>>),
    WroteWav(hound::Result<()>),
    ImportedAudio {
        file_name: Option<String>,
        result: hound::Result<DecodedAudio>,
    },
}

pub(crate) enum BackgroundTaskStatus {
    Save,
    Open,
    Export,
    Import,
}
