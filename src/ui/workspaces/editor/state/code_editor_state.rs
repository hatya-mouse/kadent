use egui_extras::syntax_highlighting::{CodeTheme, SyntectSettings};
use std::path::PathBuf;

#[derive(Default)]
pub(crate) struct CodeEditorState {
    /// The current code theme.
    pub theme: Option<CodeTheme>,
    /// Syntect settings which supports KASL language.
    pub syntect_settings: Option<SyntectSettings>,
    /// Buffer of the opened KASL programs in the code editor.
    /// This includes path to the file and the raw code.
    pub code_buffers: Vec<Option<(PathBuf, String)>>,
    // Whether the opened programs are modified or not.
    // pub are_modified: Vec<bool>,
}
