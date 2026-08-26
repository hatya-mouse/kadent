use crate::consts::{KADENT_DATA_DIR_NAME, KASL_LIB_PATH};
use std::path::{Path, PathBuf};

/// Returns the default path for KASL libraries.
pub(crate) fn default_kasl_lib_directory() -> Option<PathBuf> {
    dirs::data_dir().map(|d| d.join(KADENT_DATA_DIR_NAME).join(KASL_LIB_PATH))
}

/// Installs the KASL standard library to the specified directory.
pub(crate) fn install_kasl_stdlib(install_dir: &Path) -> std::io::Result<()> {
    let source = bundled_kasl_stdlib()?;
    copy_dir_recursive(&source, &install_dir.join("std"))
}

/// Returns a path to the bundled KASL standard library for the current operating system.
/// In debug builds, this function will return an error.
fn bundled_kasl_stdlib() -> std::io::Result<PathBuf> {
    if cfg!(debug_assertions) {
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "KASL standard library is not bundled in debug builds",
        ))
    } else {
        let exe_dir = std::env::current_exe()?;
        get_bundled_stdlib_path_from_exe_dir(&exe_dir)
    }
}

#[cfg(target_os = "macos")]
fn get_bundled_stdlib_path_from_exe_dir(exe_dir: &Path) -> std::io::Result<PathBuf> {
    exe_dir
        .parent()
        .and_then(|parent| parent.parent())
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not determine parent directory of executable",
            )
        })
        .map(|parent| parent.join("Resources").join("std"))
}

#[cfg(not(target_os = "macos"))]
fn get_bundled_stdlib_path_from_exe_dir(exe_dir: &Path) -> std::io::Result<PathBuf> {
    Ok(exe_dir.join("resources").join("std"))
}

/// Recursively copies the contents of the source directory to the destination directory.
fn copy_dir_recursive(source: &Path, destination: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(destination)?;

    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if source_path.is_dir() {
            copy_dir_recursive(&source_path, &destination_path)?;
        } else {
            std::fs::copy(&source_path, &destination_path)?;
        }
    }

    Ok(())
}
