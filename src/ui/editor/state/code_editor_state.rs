use eframe::egui;
use egui_extras::syntax_highlighting::{CodeTheme, SyntectSettings};
use std::{collections::HashMap, path::PathBuf};

#[derive(Default)]
pub(crate) struct CodeEditorState {
    /// The current code theme.
    pub theme: Option<CodeTheme>,
    /// Syntect settings which supports KASL language.
    pub syntect_settings: Option<SyntectSettings>,
    /// Open file buffers per code editor panel with stable panel ID keys.
    pub code_buffers: HashMap<egui::Id, Option<(PathBuf, String)>>,
    // Whether the opened programs are modified or not.
    // pub are_modified: Vec<bool>,
}
