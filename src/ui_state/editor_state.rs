use crate::ui_state::{
    code_editor_state::CodeEditorState, dialog_state::DialogState,
    node_graph_state::NodeGraphState, panel_layout::PanelNode, piano_roll_state::PianoRollState,
    timeline_state::TimelineState, toolbar_state::ToolbarState,
};
use kreniq_engine::{
    data_types::Beats,
    graph::node_id::NodeID,
    mixer::TrackID,
    track::{RegionID, note_track::NoteID},
};
use std::time::Instant;

#[derive(Default)]
pub struct EditorUiState {
    /// Panel layout tree.
    pub panel_layout: PanelNode,

    /// The current code editor state.
    pub code_editor_state: CodeEditorState,
    /// The current toolbar state.
    pub toolbar_state: ToolbarState,
    /// The current dialog state.
    pub dialog_state: DialogState,
    /// The current timeline state.
    pub timeline_state: TimelineState,
    /// The current piano roll state.
    pub piano_roll_state: PianoRollState,
    /// The current node graph state.
    pub node_graph_state: NodeGraphState,

    /// The current playhead position, in beats.
    pub playhead_beats: Beats,
    /// The latest playhead samples received from the audio thread.
    pub last_playhead: usize,

    /// An instant to track the last edited time for project updating.
    pub last_edit_time: Option<Instant>,

    // --- SELECTION STATE ---
    /// An ID of the currently selected track.
    pub selected_track: Option<TrackID>,
    /// An ID of the currently selected region.
    pub selected_region: Option<(TrackID, RegionID)>,
    /// An ID of the currently selected note.
    pub selected_note: Option<NoteID>,
    /// Currently selected node ID.
    pub selected_node: Option<NodeID>,
}

impl EditorUiState {
    /// Set the selected track to the given one, deselecting the note and the node.
    pub fn set_selected_track(&mut self, track_id: TrackID) {
        self.selected_track = Some(track_id);
        self.selected_region = None;

        self.selected_note = None;
        self.selected_node = None;
    }

    /// Set the selected region to the given one, deselecting the note and the node.
    pub fn set_selected_region(&mut self, track_id: TrackID, region_id: RegionID) {
        self.selected_track = Some(track_id);
        self.selected_region = Some((track_id, region_id));

        self.selected_note = None;
        self.selected_node = None;
    }

    /// Set the selected note to the given one.
    pub fn set_selected_note(&mut self, note_id: NoteID) {
        self.selected_note = Some(note_id)
    }

    /// Set the selected node to the given one.
    pub fn set_selected_node(&mut self, node_id: NodeID) {
        self.selected_node = Some(node_id);
    }

    /// Deselects the currently selected track, region, note, and node.
    pub fn deselect_all(&mut self) {
        self.selected_track = None;
        self.selected_region = None;
        self.selected_note = None;
        self.selected_node = None;
    }

    /// Deselects the currently selected region and note.
    pub fn deselect_region(&mut self) {
        self.selected_region = None;
        self.selected_note = None;
    }

    /// Deselects the currently selected note.
    pub fn deselect_note(&mut self) {
        self.selected_note = None;
    }

    /// Deselcts the currently selected node.
    pub fn deselect_node(&mut self) {
        self.selected_node = None;
    }
}
