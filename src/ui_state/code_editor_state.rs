use std::{path::PathBuf, time::Instant};

pub struct CodeEditorState {
    /// Path to the currently open file.
    pub open_file: Option<PathBuf>,
    /// `None` means the list has not been loaded yet.
    pub file_list: Option<Vec<PathBuf>>,
    /// The time when the buffer was last modified.
    pub last_edit_time: Instant,
    /// The content at the point where the file was last modified.
    pub content: String,
    /// Whether the current content has unsaved changes.
    pub dirty: bool,
}

impl Default for CodeEditorState {
    fn default() -> Self {
        Self {
            last_edit_time: Instant::now(),
            ..Default::default()
        }
    }
}
