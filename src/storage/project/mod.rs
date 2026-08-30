mod error;
mod init;
mod new_project;
mod open_project;
mod serial;

pub(crate) use error::SaveError;
pub(crate) use new_project::create_new_project;
pub(crate) use open_project::open_project_to_ctx;

use crate::consts::KADENT_FILE_VERSION;
use crate::core::audio_engine::mixer::ProjectData;
use crate::storage::project::serial::{DecodableProject, EncodableProject};
use crate::{core::metadata::ProjectMeta, storage::project::error::LoadError};
use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

/// Saves the given project to the given path. Returns an error if the file cannot be created or written to.
pub(crate) fn save_project(
    path: &Path,
    data: &ProjectData,
    meta: &ProjectMeta,
) -> Result<(), SaveError> {
    let tmp_path = temp_path_for(path);

    {
        let mut file = File::create(&tmp_path).map_err(SaveError::IoError)?;

        // Write the project data to the file
        // First write "KADENT" to check if the file is a Kadent ProjectData file when opened
        file.write_all("KADENT".as_bytes())
            .map_err(SaveError::IoError)?;

        // Then write the version of Kadent
        let major_ver: u32 = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
        let minor_ver: u32 = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
        let patch_ver: u32 = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();
        let file_ver: u64 = KADENT_FILE_VERSION;
        file.write_all(&major_ver.to_le_bytes())
            .map_err(SaveError::IoError)?;
        file.write_all(&minor_ver.to_le_bytes())
            .map_err(SaveError::IoError)?;
        file.write_all(&patch_ver.to_le_bytes())
            .map_err(SaveError::IoError)?;
        file.write_all(&file_ver.to_le_bytes())
            .map_err(SaveError::IoError)?;

        let project_file = EncodableProject { data, meta };
        let data_bytes = sode::encode(&project_file).map_err(SaveError::EncodeError)?;
        file.write_all(&data_bytes).map_err(SaveError::IoError)?;
        file.flush().map_err(SaveError::IoError)?;
    }

    // Atomically replace the real project file only once the temp file is fully written
    // so that the project file is not incompletely written even if the program is quit mid-write
    std::fs::rename(&tmp_path, path).map_err(SaveError::IoError)?;

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
fn load_project(path: &Path) -> Result<DecodableProject, LoadError> {
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
    let mut file_ver_bytes = [0u8; 8];
    file.read_exact(&mut major_bytes)
        .map_err(LoadError::IoError)?;
    file.read_exact(&mut minor_bytes)
        .map_err(LoadError::IoError)?;
    file.read_exact(&mut patch_bytes)
        .map_err(LoadError::IoError)?;
    file.read_exact(&mut file_ver_bytes)
        .map_err(LoadError::IoError)?;
    // let file_major_ver = u32::from_le_bytes(major_bytes);
    // let file_minor_ver = u32::from_le_bytes(minor_bytes);
    // let file_patch_ver = u32::from_le_bytes(patch_bytes);
    let file_ver = u64::from_le_bytes(file_ver_bytes);

    // Read the rest of the file and parse the payload with postcard
    let mut payload = Vec::new();
    file.read_to_end(&mut payload).map_err(LoadError::IoError)?;
    let decodable_project: DecodableProject =
        sode::decode(&payload, file_ver).map_err(LoadError::DecodeError)?;

    println!(
        "Loaded Project: {:#?} (created with Kadent version {}.{}.{} and file version {})",
        decodable_project,
        u32::from_le_bytes(major_bytes),
        u32::from_le_bytes(minor_bytes),
        u32::from_le_bytes(patch_bytes),
        file_ver
    );

    Ok(decodable_project)
}

pub(crate) fn get_project_dir(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .and_then(|p| p.canonicalize().ok())
        .unwrap_or_else(|| PathBuf::from("."))
}
