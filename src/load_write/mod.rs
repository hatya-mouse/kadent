mod audio_ctx_io;
mod error;
mod graph_io;
mod init;
mod load_proj_res;
mod node_io;
mod project_io;
mod project_meta_io;
mod tempo_map_io;
mod track_io;
mod traits;

use crate::{load_write::error::LoadError, metadata::ProjectMeta};
pub(crate) use init::init_kasl_nodes;
pub(crate) use load_proj_res::LoadProjResult;
pub(crate) use project_meta_io::{StoredProjMeta, StoredTrackMeta};
pub(crate) use traits::{AsBytes, FromBytes};

use crate::load_write::traits::safe_read;
use kreniq_engine::mixer::Project;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

/// Saves the project into a directory, creating it if it doesn't exist.
/// The main project file is written as `project.knq` inside `dir_path`.
pub(crate) fn save_project_to_dir(
    dir_path: &Path,
    project: &Project,
    project_meta: &ProjectMeta,
) -> std::io::Result<()> {
    // Create the project directory
    fs::create_dir_all(dir_path)?;
    // Create a kasl/ directory where KASL programs can be saved
    let kasl_dir_path = dir_path.join("kasl");
    fs::create_dir_all(kasl_dir_path)?;
    // Write the project file inside the project directory
    let file_path = dir_path.join("project.knq");
    save_project(&file_path, project, project_meta)
}

/// Loads a project from a project directory, reading `project.knq` inside it.
pub(crate) fn load_project_from_dir(dir_path: &Path) -> Result<LoadProjResult, LoadError> {
    load_project(&dir_path.join("project.knq"))
}

/// Saves the given project to the given path. Returns an error if the file cannot be created or written to.
pub(crate) fn save_project(
    path: &Path,
    project: &Project,
    project_meta: &ProjectMeta,
) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    // Write the project data to the file
    // First write "KNODIQ" to check if the file is a Kreniq Project file
    file.write_all("KNODIQ".as_bytes())?;

    // Then write the version of Kreniq
    let major_ver: u32 = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
    let minor_ver: u32 = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
    let patch_ver: u32 = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();
    file.write_all(&major_ver.to_le_bytes())?;
    file.write_all(&minor_ver.to_le_bytes())?;
    file.write_all(&patch_ver.to_le_bytes())?;

    // Write the project metadata
    let stored_proj_meta = StoredProjMeta::from_proj_meta(project_meta);
    let mut proj_meta_bytes = Vec::new();
    stored_proj_meta.as_bytes(&mut proj_meta_bytes);
    // Write the length of the project metadata before writing the project metadata itself
    file.write_all(&(proj_meta_bytes.len() as u64).to_le_bytes())?;
    file.write_all(&proj_meta_bytes)?;

    // Write the project
    let mut project_bytes = Vec::new();
    project.as_bytes(&mut project_bytes);

    file.write_all(&project_bytes)?;
    file.flush()?;

    Ok(())
}

/// Loads a project file from the given path. Returns an error if the file is not a Kreniq Project file or if the file is corrupted.
pub(crate) fn load_project(path: &Path) -> Result<LoadProjResult, LoadError> {
    // Load the file from the path
    let mut file = File::open(path).map_err(LoadError::IoError)?;

    // Read the first 6 bytes to check if it's a Kreniq Project file
    let mut header_bytes = [0u8; 6];
    file.read_exact(&mut header_bytes)
        .map_err(LoadError::IoError)?;

    if &header_bytes != b"KNODIQ" {
        return Err(LoadError::NotAProjectFile);
    }

    // Read the next 12 bytes to get the version of Kreniq that created the project
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

    // Read the rest of the file and parse the project
    let mut project_bytes = Vec::new();
    file.read_to_end(&mut project_bytes)
        .map_err(LoadError::IoError)?;
    let result = LoadProjResult::from_bytes(&project_bytes).map_err(LoadError::FileCorrupted)?;

    Ok(result)
}
