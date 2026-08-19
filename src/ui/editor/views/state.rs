use crate::ui::editor::{
    CodeEditorState, DialogState, NodeGraphState, PanelView, PianoRollState, StatusBarState,
    TimelineState, toolbar::ToolbarState, views::PanelViewState,
};
use kadent_engine::thread::AudioError;
use std::collections::{HashMap, hash_map::Entry};
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

    // --- PANEL-SPECIFIC STATES ---
    /// The states for each panel, keyed by their unique ID.
    pub panel_states: HashMap<Uuid, PanelViewState>,

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
    pub(crate) fn get_panel_state_or_insert<F>(
        &mut self,
        panel_id: Uuid,
        desired_view: PanelView,
        default_state: F,
    ) -> &mut PanelViewState
    where
        F: FnOnce() -> PanelViewState,
    {
        match self.panel_states.entry(panel_id) {
            Entry::Occupied(mut entry) => {
                if entry.get().view() != desired_view {
                    entry.insert(default_state());
                }
                entry.into_mut()
            }
            Entry::Vacant(entry) => entry.insert(default_state()),
        }
    }

    pub(crate) fn insert_panel_state(&mut self, panel_id: Uuid, state: PanelViewState) {
        self.panel_states.insert(panel_id, state);
    }

    pub(crate) fn remove_panel_states(&mut self, panel_ids: &[Uuid]) {
        for panel_id in panel_ids {
            self.panel_states.remove(panel_id);
        }
    }
}
