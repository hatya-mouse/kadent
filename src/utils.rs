//! Utilities not strictly tied to Kadent.

#[cfg(target_os = "macos")]
use std::path::Path;
use std::path::PathBuf;

/// Spawns a background thread to initialize the value.
/// The value must implement `Default`. The macro returns an `Arc<Mutex<T>>` where `T` is the type of the value being initialized.
#[macro_export]
macro_rules! spawn_background_init {
    ($init_expr:expr) => {{
        let data = Arc::new(Mutex::new(Default::default()));
        let data_clone = Arc::clone(&data);

        // Spawn a background thread to initialize the value
        thread::spawn(move || {
            let loaded = $init_expr;
            if let Ok(mut guard) = data_clone.lock() {
                *guard = loaded;
            }
        });

        data
    }};
}

/// Generates a version number string.
pub(crate) fn version_string() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!("Version: {version}")
}

/// Returns a path to the resources directory in the package for the current operating system.
/// In debug builds, this function will return an error.
pub(crate) fn get_resources_path() -> std::io::Result<PathBuf> {
    let exe_dir = std::env::current_exe()?;
    if cfg!(debug_assertions) {
        exe_dir.join("resources").canonicalize()
    } else {
        get_resources_path_from(&exe_dir)
    }
}

#[cfg(target_os = "macos")]
fn get_resources_path_from(exe_dir: &Path) -> std::io::Result<PathBuf> {
    exe_dir
        .parent()
        .and_then(|parent| parent.parent())
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not determine parent directory of executable",
            )
        })
        .map(|parent| parent.join("Resources"))
}

#[cfg(not(target_os = "macos"))]
fn get_resources_path_from(exe_dir: &Path) -> std::io::Result<PathBuf> {
    Ok(exe_dir.join("resources"))
}
