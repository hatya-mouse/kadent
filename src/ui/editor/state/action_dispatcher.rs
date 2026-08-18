use crate::{actions::EditorAction, background_thread::BackgroundThreadHandle};
use std::{collections::VecDeque, path::PathBuf, time::Instant};

pub(crate) struct ActionDispatcher {
    /// A thread handle to communicate with the background processing thread.
    pub background_handle: BackgroundThreadHandle,
    /// Pending actions to be executed at the end of the frame.
    pub pending: VecDeque<EditorAction>,
    /// An instant to track the last edited time for project updating.
    pub last_edit_time: Option<Instant>,

    // --- EXPORT ---
    /// The path to export the project to last time the export button was clicked.
    pub pending_export_path: Option<PathBuf>,
}

impl ActionDispatcher {
    pub(crate) fn new(background_handle: BackgroundThreadHandle) -> Self {
        Self {
            background_handle,
            pending: VecDeque::new(),
            last_edit_time: None,
            pending_export_path: None,
        }
    }
}
