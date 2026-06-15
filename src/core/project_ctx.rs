use crate::core::metadata::ProjectMeta;
use kadent_engine::{data_types::AudioContext, mixer::Project};
use std::path::PathBuf;

/// A struct that holds the context of the editor, including the project context and the audio context.
pub struct EditorContext {
    pub proj_ctx: ProjectContext,
    pub audio_ctx: AudioContext,
}

impl EditorContext {
    pub fn new(proj_ctx: ProjectContext, audio_ctx: AudioContext) -> Self {
        Self {
            proj_ctx,
            audio_ctx,
        }
    }
}

pub struct ProjectContext {
    /// The path to the project file.
    pub project_path: PathBuf,
    /// A master source of the project.
    pub project: Project,
    /// The metadata of the project.
    pub project_meta: ProjectMeta,
}

impl ProjectContext {
    pub fn new(project_path: PathBuf, project: Project, project_meta: ProjectMeta) -> Self {
        Self {
            project_path,
            project,
            project_meta,
        }
    }
}
