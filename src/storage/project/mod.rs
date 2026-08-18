mod error;
mod init;
mod new_project;
mod open_project;
mod stored;

pub(crate) use init::init_kasl_nodes;
pub(crate) use new_project::create_new_project;
pub(crate) use open_project::open_project_to_ctx;
pub(crate) use stored::StoredTrackMeta;

use crate::{
    core::metadata::ProjectMeta,
    storage::project::{
        error::LoadError,
        stored::{StoredProjMeta, StoredProject},
    },
};
use kadent_engine::mixer::ProjectData;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

pub(crate) struct LoadProjResult {
    pub(crate) project: ProjectData,
    pub(crate) project_meta: StoredProjMeta,
}

/// The full payload written after the "KADENT" + version header.
#[derive(Serialize, Deserialize)]
struct StoredProjectFile {
    project_meta: StoredProjMeta,
    project: StoredProject,
}

/// Saves the given project to the given path. Returns an error if the file cannot be created or written to.
///
/// Writes to a temporary file first and renames it over `path` at the end, so a crash or a
/// forced quit mid-write can never leave a half-written project file at `path`.
pub(crate) fn save_project(
    path: &Path,
    project: &ProjectData,
    project_meta: &ProjectMeta,
) -> std::io::Result<()> {
    let tmp_path = temp_path_for(path);

    {
        let mut file = File::create(&tmp_path)?;

        // Write the project data to the file
        // First write "KADENT" to check if the file is a Kadent ProjectData file when opened
        file.write_all("KADENT".as_bytes())?;

        // Then write the version of Kadent
        let major_ver: u32 = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
        let minor_ver: u32 = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
        let patch_ver: u32 = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();
        file.write_all(&major_ver.to_le_bytes())?;
        file.write_all(&minor_ver.to_le_bytes())?;
        file.write_all(&patch_ver.to_le_bytes())?;

        // Serialize the metadata and the project together with postcard
        let stored = StoredProjectFile {
            project_meta: StoredProjMeta::from_project_meta(project_meta),
            project: StoredProject::from_project(project),
        };
        let payload = postcard::to_allocvec(&stored)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        file.write_all(&payload)?;
        file.flush()?;
    }

    // Atomically replace the real project file only once the temp file is fully written
    // so that the project file is not incompletely written even if the program is quit mid-write
    std::fs::rename(&tmp_path, path)?;

    Ok(())
}

/// Returns a sibling path with `.tmp` appended to the file name, used as the staging file for
/// an atomic save.
fn temp_path_for(path: &Path) -> PathBuf {
    let mut file_name = path
        .file_name()
        .map(|n| n.to_os_string())
        .unwrap_or_default();
    file_name.push(".tmp");
    path.with_file_name(file_name)
}

/// Loads a project file from the given path. Returns an error if the file is not a Kadent ProjectData file or if the file is corrupted.
pub(crate) fn load_project(path: &Path) -> Result<LoadProjResult, LoadError> {
    // Load the file from the path
    let mut file = File::open(path).map_err(LoadError::IoError)?;

    // Read the first 6 bytes to check if it's a Kadent ProjectData file
    let mut header_bytes = [0u8; 6];
    file.read_exact(&mut header_bytes)
        .map_err(LoadError::IoError)?;

    if &header_bytes != b"KADENT" {
        return Err(LoadError::NotAProjectFile);
    }

    // Read the next 12 bytes to get the version of Kadent that created the project
    let mut major_bytes = [0u8; 4];
    let mut minor_bytes = [0u8; 4];
    let mut patch_bytes = [0u8; 4];
    file.read_exact(&mut major_bytes)
        .map_err(LoadError::IoError)?;
    file.read_exact(&mut minor_bytes)
        .map_err(LoadError::IoError)?;
    file.read_exact(&mut patch_bytes)
        .map_err(LoadError::IoError)?;
    // let file_major_ver = u32::from_le_bytes(major_bytes);
    // let file_minor_ver = u32::from_le_bytes(minor_bytes);
    // let file_patch_ver = u32::from_le_bytes(patch_bytes);

    // Read the rest of the file and parse the payload with postcard
    let mut payload = Vec::new();
    file.read_to_end(&mut payload).map_err(LoadError::IoError)?;
    let stored: StoredProjectFile = postcard::from_bytes(&payload).map_err(LoadError::Postcard)?;

    Ok(LoadProjResult {
        project: stored.project.to_project(),
        project_meta: stored.project_meta,
    })
}

pub(crate) fn get_project_dir(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .and_then(|p| p.canonicalize().ok())
        .unwrap_or_else(|| PathBuf::from("."))
}
