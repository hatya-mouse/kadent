use crate::ui::editor::{
    AutomationState, CodeEditorView, DialogState, ErrorListView, InspectorView, NodeGraphState,
    PanelViewState, PianoRollState, StatusBarView, TimelineState, ToolbarState,
};
use std::collections::HashMap;
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
    /// The code editor view.
    pub(crate) code_editor: CodeEditorView,
    /// The error list view.
    pub(crate) error_list: ErrorListView,
    /// The inspector view.
    pub(crate) inspector: InspectorView,
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
    pub(crate) status_bar: StatusBarView,
}

impl ViewStates {
    pub(crate) fn remove_panel_states(&mut self, panel_ids: &[Uuid]) {
        for panel_id in panel_ids {
            self.panel_states.remove(panel_id);
        }
    }
}
