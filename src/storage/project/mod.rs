mod error;
mod init;
mod new_project;
mod trait_impl;
mod traits;

pub(crate) use init::init_kasl_nodes;
pub(crate) use new_project::create_new_project;
pub(crate) use trait_impl::load_proj_res::LoadProjResult;
pub(crate) use trait_impl::project_meta::StoredTrackMeta;
pub(crate) use traits::{AsBytes, FromBytes, safe_read};

use crate::{
    core::metadata::ProjectMeta,
    storage::project::{error::LoadError, trait_impl::project_meta::StoredProjMeta},
};
use kadent_engine::mixer::Project;
use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

/// Saves the given project to the given path. Returns an error if the file cannot be created or written to.
pub(crate) fn save_project(
    path: &Path,
    project: &Project,
    project_meta: &ProjectMeta,
) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    // Write the project data to the file
    // First write "KADENT" to check if the file is a Kadent Project file
    file.write_all("KADENT".as_bytes())?;

    // Then write the version of Kadent
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

/// Loads a project file from the given path. Returns an error if the file is not a Kadent Project file or if the file is corrupted.
pub(crate) fn load_project(path: &Path) -> Result<LoadProjResult, LoadError> {
    // Load the file from the path
    let mut file = File::open(path).map_err(LoadError::IoError)?;

    // Read the first 6 bytes to check if it's a Kadent Project file
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

    // Read the rest of the file and parse the project
    let mut project_bytes = Vec::new();
    file.read_to_end(&mut project_bytes)
        .map_err(LoadError::IoError)?;
    let result = LoadProjResult::from_bytes(&project_bytes).map_err(LoadError::FileCorrupted)?;

    Ok(result)
}

pub(crate) fn get_project_dir(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .and_then(|p| p.canonicalize().ok())
        .unwrap_or_else(|| PathBuf::from("."))
}
