use egui_extras::syntax_highlighting::{CodeTheme, SyntectSettings};

#[derive(Default)]
pub(crate) struct CodeEditorState {
    /// The current code theme.
    pub theme: Option<CodeTheme>,
    /// Syntect settings which supports KASL language.
    pub syntect_settings: Option<SyntectSettings>,
}
