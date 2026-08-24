//! Defines constant values used across the application.

use eframe::egui;

// --- APP INFO & DATA PATH ---

/// The name of the Kadent data directory.
pub(crate) const KADENT_DATA_DIR_NAME: &str = "Kadent";
/// The name of the application.
pub(crate) const APP_NAME: &str = "Kadent";
/// The file extension for Kadent project files.
pub(crate) const PROJECT_FILE_EXTENSION: &str = "kdp";
/// The current file version of Kadent project files.
pub(crate) const KADENT_FILE_VERSION: u64 = 0;

/// A relative path to recent projects data file. Relative to `dirs::data_dir()`.
pub(crate) const RECENT_PROJCETS_PATH: &str = "recent_projects.json";
/// The maximum number of recent projects shown in the splash screen.
pub(crate) const RECENT_PROJCETS_MAX_NUM: usize = 20;

// --- WAVEFORM LOD ---

/// The small block size to get the peaks when generating waveform.
pub(crate) const SMALL_BLOCK_SIZE: usize = 64;
/// The medium block size to get the peaks when generating waveform.
pub(crate) const MEDIUM_BLOCK_SIZE: usize = 1024;
/// The large block size to get the peaks when generating waveform.
pub(crate) const LARGE_BLOCK_SIZE: usize = 8192;

// --- AUDIO ---

/// The default number of output channels used when creating a project
/// or falling back from an invalid/corrupted `PlaybackContext`.
pub(crate) const DEFAULT_CHANNELS: usize = 2;
/// The default sample rate (Hz) used when creating a project or falling
/// back from an invalid/corrupted `PlaybackContext`.
pub(crate) const DEFAULT_SAMPLE_RATE: u64 = 48000;
/// The default audio buffer size used when creating a project or falling
/// back from an invalid/corrupted `PlaybackContext`.
pub(crate) const DEFAULT_BUFFER_SIZE: usize = 512;

// --- PANEL ---

/// The recommended height of the header inside each panels.
pub(crate) const PANEL_HEADER_HEIGHT: f32 = 36.0;
/// The default margin of the panel header.
pub(crate) const PANEL_HEADER_MARGIN: egui::Margin = egui::Margin::symmetric(8, 6);

// --- TIMELINE ---

/// The height of the timeline scroll bar.
pub(crate) const SCROLL_BAR_HEIGHT: f32 = 12.0;
/// The minimum height of the track row in pixels.
pub(crate) const MIN_TRACK_HEIGHT: f32 = 30.0;
/// The maximum height of the track row in pixels.
pub(crate) const MAX_TRACK_HEIGHT: f32 = 200.0;
/// Extra pixels of empty space inserted before zero beat.
pub(crate) const TIMELINE_LEFT_PADDING: f32 = 50.0;
/// Extra pixels of empty space appended after the last region or project range end.
pub(crate) const TIMELINE_RIGHT_PADDING: f32 = 800.0;
/// The minimum pixels per beat.
pub(crate) const TIMELINE_MIN_PPB: f32 = 1.0;
/// The maximum pixels per beat.
pub(crate) const TIMELINE_MAX_PPB: f32 = 4000.0;

// --- UI ---
/// The minimum width of the sidebar in pixels.
pub(crate) const MIN_SIDEBAR_WIDTH: f32 = 100.0;
/// The maximum width of the sidebar in pixels.
pub(crate) const MAX_SIDEBAR_WIDTH: f32 = 800.0;
