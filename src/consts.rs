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
