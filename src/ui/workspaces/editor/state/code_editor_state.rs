use egui_extras::syntax_highlighting::{CodeTheme, SyntectSettings};
use std::path::PathBuf;

#[derive(Default)]
pub(crate) struct CodeEditorState {
    /// The current code theme.
    pub theme: Option<CodeTheme>,
    /// Syntect settings which supports KASL language.
    pub syntect_settings: Option<SyntectSettings>,
    /// Opened KASL programs in the code editor.
    pub opened_programs: Vec<PathBuf>,
    /// Buffer of the opened KASL programs in the code editor.
    pub code_buffers: Vec<String>,
    // Whether the opened programs are modified or not.
    // pub are_modified: Vec<bool>,
}
