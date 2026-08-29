use crate::{
    consts::{KADENT_DATA_DIR_NAME, KASL_LIB_PATH},
    utils::get_resources_path,
};
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
fn bundled_kasl_stdlib() -> std::io::Result<PathBuf> {
    get_resources_path().map(|path| path.join("std"))
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
