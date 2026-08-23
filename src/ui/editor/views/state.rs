use crate::core::audio_engine::thread::AudioError;
use crate::ui::editor::{
    AutomationState, CodeEditorState, DialogState, NodeGraphState, PanelView, PianoRollState,
    StatusBarState, TimelineState, toolbar::ToolbarState, views::PanelViewState,
};
use std::collections::{HashMap, hash_map::Entry};
use uuid::Uuid;

#[derive(Default)]
pub(crate) struct ViewStates {
    // --- PANELS ---
    /// The current timeline state.
    pub(crate) timeline: TimelineState,
    /// The current piano roll state.
    pub(crate) piano_roll: PianoRollState,
    /// The current node graph state.
    pub(crate) node_graph: NodeGraphState,
    /// The current code editor state.
    pub(crate) code_editor: CodeEditorState,
    /// the current automation editor state.
    pub(crate) automation: AutomationState,

    // --- PANEL-SPECIFIC STATES ---
    /// The states for each panel, keyed by their unique ID.
    pub(crate) panel_states: HashMap<Uuid, PanelViewState>,

    // --- NON-PANEL STATES ---
    /// The current toolbar state.
    pub(crate) toolbar: ToolbarState,
    /// The current dialog state.
    pub(crate) dialog: DialogState,
    /// The current status bar state.
    pub(crate) status_bar: StatusBarState,

    // --- AUDIO ERRORS ---
    /// Errors to be shown.
    pub(crate) errors: Vec<AudioError>,
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
