use crate::core::metadata::ProjectMeta;
use kadent_engine::{data_types::ProjectConfig, mixer::Project};
use std::path::PathBuf;

/// A struct that holds the context of the editor, including the project context and the audio context.
pub struct EditorContext {
    pub proj_ctx: ProjectContext,
    pub proj_config: ProjectConfig,
}

impl EditorContext {
    pub fn new(proj_ctx: ProjectContext, proj_config: ProjectConfig) -> Self {
        Self {
            proj_ctx,
            proj_config,
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
