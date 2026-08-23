use crate::core::audio_engine::mixer::ProjectData;
use crate::core::metadata::ProjectMeta;
use std::path::PathBuf;

pub(crate) struct ProjectContext {
    /// The path to the project file.
    pub(crate) path: PathBuf,
    /// A master source of the project.
    pub(crate) data: ProjectData,
    /// The metadata of the project.
    pub(crate) meta: ProjectMeta,
}

impl ProjectContext {
    pub(crate) fn new(path: PathBuf, data: ProjectData, meta: ProjectMeta) -> Self {
        Self { path, data, meta }
    }
}
