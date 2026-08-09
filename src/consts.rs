//! Defines constant values used across the application.

use eframe::egui;

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

/// The default number of output channels used when creating a project
/// or falling back from an invalid/corrupted `PlaybackContext`.
pub(crate) const DEFAULT_CHANNELS: usize = 2;
/// The default sample rate (Hz) used when creating a project or falling
/// back from an invalid/corrupted `PlaybackContext`.
pub(crate) const DEFAULT_SAMPLE_RATE: u64 = 48000;
/// The default audio buffer size used when creating a project or falling
/// back from an invalid/corrupted `PlaybackContext`.
pub(crate) const DEFAULT_BUFFER_SIZE: usize = 512;
/// The default maximum voice count used when creating a project or
/// falling back from an invalid/corrupted `PlaybackContext`.
pub(crate) const DEFAULT_MAX_VOICES: usize = 32;

/// The recommended height of the header inside each panels.
pub(crate) const PANEL_HEADER_HEIGHT: f32 = 36.0;
/// The default margin of the panel header.
pub(crate) const PANEL_HEADER_MARGIN: egui::Margin = egui::Margin::symmetric(8, 4);
