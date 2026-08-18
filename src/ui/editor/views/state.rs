use crate::ui::editor::{
    CodeEditorState, DialogState, NodeGraphState, PianoRollState, StatusBarState, TimelineState,
    toolbar::ToolbarState,
};
use kadent_engine::thread::AudioError;
use uuid::Uuid;

#[derive(Default)]
pub(crate) struct ViewStates {
    // --- PANELS ---
    /// The current timeline state.
    pub timeline: TimelineState,
    /// The current piano roll state.
    pub piano_roll: PianoRollState,
    /// The current node graph state.
    pub node_graph: NodeGraphState,
    /// The current code editor state.
    pub code_editor: CodeEditorState,

    // --- NON-PANEL STATES ---
    /// The current toolbar state.
    pub toolbar: ToolbarState,
    /// The current dialog state.
    pub dialog: DialogState,
    /// The current status bar state.
    pub status_bar: StatusBarState,

    // --- AUDIO ERRORS ---
    /// Errors to be shown.
    pub errors: Vec<AudioError>,
}

impl ViewStates {
    pub(crate) fn remove_panel_states(&mut self, panel_ids: &[Uuid]) {
        for panel_id in panel_ids {
            self.timeline.remove_panel_state(panel_id);
            self.piano_roll.remove_panel_state(panel_id);
            self.code_editor.remove_panel_state(panel_id);
        }
    }
}
