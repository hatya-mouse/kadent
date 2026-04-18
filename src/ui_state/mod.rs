pub mod dialog_state;
pub mod editor_state;
pub mod piano_roll_state;
pub mod timeline_state;

use crate::ui_state::editor_state::EditorUIState;

#[derive(Default)]
pub enum AppScreen {
    /// The welcome screen shown on app launch.
    #[default]
    Splash,
    /// The main editor screen.
    Editor,
}

#[derive(Default)]
pub struct KnodiqUIState {
    /// The currently opened screen.
    pub app_screen: AppScreen,

    /// The state of the editor UI.
    pub editor_state: EditorUIState,
}
