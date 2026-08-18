use crate::core::metadata::ProjectMeta;
use kadent_engine::mixer::ProjectData;
use std::path::PathBuf;

pub struct ProjectContext {
    /// The path to the project file.
    pub path: PathBuf,
    /// A master source of the project.
    pub data: ProjectData,
    /// The metadata of the project.
    pub meta: ProjectMeta,
}

impl ProjectContext {
    pub fn new(path: PathBuf, data: ProjectData, meta: ProjectMeta) -> Self {
        Self { path, data, meta }
    }
}
