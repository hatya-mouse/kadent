use crate::ui::editor::actions::FileNode;
use egui_extras::syntax_highlighting::SyntectSettings;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use uuid::Uuid;

#[derive(Default)]
pub(crate) struct CodeEditorState {
    /// Syntect settings which supports KASL language.
    pub(crate) syntect_settings: Option<Arc<SyntectSettings>>,
    /// Open file buffers per code editor panel with stable panel ID keys.
    pub(crate) code_buffers: HashMap<Uuid, Option<(PathBuf, String)>>,

    /// Cached graph of the project directory structure.
    pub(crate) project_dir_cache: Vec<FileNode>,
    // Whether the opened programs are modified or not.
    // pub(crate) are_modified: Vec<bool>,
}
