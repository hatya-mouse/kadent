use std::time::Duration;

pub(crate) mod audio_data;
pub(crate) mod data_types;
pub(crate) mod graph;
pub(crate) mod mixer;
pub(crate) mod node;
pub(crate) mod thread;
pub(crate) mod timing;
pub(crate) mod track;
pub(crate) mod utils;

/// Maximum supported number of channels for audio output.
pub(crate) const MAX_CHANNELS: usize = 16;
/// Number of events to be processed in a single frame.
pub(crate) const MAX_EVENTS: usize = 4;
/// Duration to wait in threads.
pub(crate) const THREAD_WAIT_DURATION: Duration = Duration::from_millis(5);
