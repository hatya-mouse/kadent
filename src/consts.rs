//! Defines constant values used across the application.

/// The name of the Kadent data directory.
pub(crate) const KADENT_DATA_DIR_NAME: &str = "Kadent";
/// The name of the application.
pub(crate) const APP_NAME: &str = "Kadent";
/// The file extension for Kadent project files.
pub(crate) const PROJECT_FILE_EXTENSION: &str = "kdp";

/// A relative path to recent projects data file. Relative to `dirs::data_dir()`.
pub(crate) const RECENT_PROJCETS_PATH: &str = "recent_projects.json";
/// The maximum number of recent projects shown in the splash screen.
pub(crate) const RECENT_PROJCETS_MAX_NUM: usize = 20;

/// The small block size to get the peaks when generating waveform.
pub(crate) const SMALL_BLOCK_SIZE: usize = 64;
/// The medium block size to get the peaks when generating waveform.
pub(crate) const MEDIUM_BLOCK_SIZE: usize = 1024;
/// The large block size to get the peaks when generating waveform.
pub(crate) const LARGE_BLOCK_SIZE: usize = 8192;

/// The recommended height of the header inside each panels.
pub(crate) const PANEL_HEADER_HEIGHT: f32 = 28.0;
