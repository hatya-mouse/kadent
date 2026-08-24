use crate::{
    background_thread::{BackgroundThreadCommand, BackgroundThreadHandle},
    ui::editor::actions::EditorAction,
};
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

    /// Pushes a editor action to the pending queue to be executed at the end of the frame.
    pub(crate) fn push_action(&mut self, action: EditorAction) {
        self.pending.push_back(action);
    }

    /// Marks the project as modified and updates the last edit time. Should be called whenever the project is modified.
    pub(crate) fn modified_project(&mut self) {
        self.last_edit_time = Some(Instant::now());
    }

    /// Pushes a command to the background thread for processing.
    pub(crate) fn push_background_job(&mut self, command: BackgroundThreadCommand) {
        self.background_handle.command_tx.send(command).ok();
    }
}
