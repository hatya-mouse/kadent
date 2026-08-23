use crate::{background_thread::BackgroundThreadHandle, ui::editor::actions::EditorAction};
use std::{collections::VecDeque, path::PathBuf, time::Instant};

pub(crate) struct ActionDispatcher {
    /// A thread handle to communicate with the background processing thread.
    pub(crate) background_handle: BackgroundThreadHandle,
    /// Pending actions to be executed at the end of the frame.
    pub(crate) pending: VecDeque<EditorAction>,
    /// An instant to track the last edited time for project updating.
    pub(crate) last_edit_time: Option<Instant>,

    // --- EXPORT ---
    /// The path to export the project to last time the export button was clicked.
    pub(crate) pending_export_path: Option<PathBuf>,
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

    pub(crate) fn push_action(&mut self, action: EditorAction) {
        self.pending.push_back(action);
    }
}
