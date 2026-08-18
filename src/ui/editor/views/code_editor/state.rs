use crate::ui::editor::actions::FileNode;
use egui_extras::syntax_highlighting::{CodeTheme, SyntectSettings};
use std::{collections::HashMap, path::PathBuf};
use uuid::Uuid;

#[derive(Default)]
pub(crate) struct CodeEditorState {
    /// The current code theme.
    pub theme: Option<CodeTheme>,
    /// Syntect settings which supports KASL language.
    pub syntect_settings: Option<SyntectSettings>,
    /// Open file buffers per code editor panel with stable panel ID keys.
    pub code_buffers: HashMap<Uuid, Option<(PathBuf, String)>>,

    /// Cached graph of the project directory structure.
    pub project_dir_cache: Vec<FileNode>,
    // Whether the opened programs are modified or not.
    // pub are_modified: Vec<bool>,
}
