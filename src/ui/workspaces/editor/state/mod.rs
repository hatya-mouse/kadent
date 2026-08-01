mod code_editor_state;
mod dialog_state;
mod modification;
mod node_graph_state;
mod panel_layout;
mod piano_roll_state;
mod status_bar_state;
mod timeline_state;
mod toolbar_state;

pub(super) use dialog_state::{AddTrackState, DialogState};
pub(super) use modification::Modification;
pub(super) use panel_layout::{PanelNode, PanelView, SplitDir};
pub(super) use status_bar_state::TempStatusNotification;

use crate::{
    actions::FileNode,
    ui::workspaces::editor::state::{
        code_editor_state::CodeEditorState, node_graph_state::NodeGraphState,
        piano_roll_state::PianoRollState, status_bar_state::StatusBarState,
        timeline_state::TimelineState, toolbar_state::ToolbarState,
    },
};
use kadent_engine::{
    data_types::{AudioContext, Ticks},
    graph::node_id::NodeID,
    mixer::TrackID,
    track::{RegionID, note_track::NoteID},
};
use midir::{MidiInput, MidiInputPorts};
use std::time::Instant;

#[derive(Default)]
pub(crate) enum Selection {
    #[default]
    None,
    Track(TrackID),
    Region(TrackID, RegionID),
    Node(TrackID, NodeID),
    Note(TrackID, RegionID, NoteID),
}

impl Selection {
    pub(crate) fn track_id(&self) -> Option<TrackID> {
        match self {
            Selection::Track(track_id) => Some(*track_id),
            Selection::Region(track_id, _) => Some(*track_id),
            Selection::Node(track_id, _) => Some(*track_id),
            Selection::Note(track_id, _, _) => Some(*track_id),
            Selection::None => None,
        }
    }

    pub(crate) fn region_id(&self) -> Option<RegionID> {
        match self {
            Selection::Region(_, region_id) => Some(*region_id),
            Selection::Note(_, region_id, _) => Some(*region_id),
            _ => None,
        }
    }

    pub(crate) fn note_id(&self) -> Option<NoteID> {
        match self {
            Selection::Note(_, _, note_id) => Some(*note_id),
            _ => None,
        }
    }

    pub(crate) fn node_id(&self) -> Option<NodeID> {
        match self {
            Selection::Node(_, node_id) => Some(*node_id),
            _ => None,
        }
    }

    pub(crate) fn track_and_region_id(&self) -> Option<(TrackID, RegionID)> {
        match self {
            Selection::Region(track_id, region_id) => Some((*track_id, *region_id)),
            Selection::Note(track_id, region_id, _) => Some((*track_id, *region_id)),
            _ => None,
        }
    }
}

#[derive(Default)]
pub(crate) struct EditorUiState {
    /// Panel layout tree.
    pub panel_layout: PanelNode,

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
    /// The current code editor state.
    pub code_editor_state: CodeEditorState,
    /// The current status bar state.
    pub status_bar_state: StatusBarState,

    /// Whether the audio is playing.
    pub is_playing: bool,
    /// The current playhead position, in beats.
    pub playhead_ticks: Ticks,
    /// The latest playhead samples received from the audio thread.
    pub last_playhead: usize,

    /// An instant to track the last edited time for project updating.
    pub last_edit_time: Option<Instant>,

    // --- SELECTION STATE ---
    /// Currently selected content.
    pub selection: Selection,

    // --- MODIFICATION STATE ---
    /// The last modified value in purpose of showing the value in the status bar.
    pub modification: Modification,

    // --- PROJECT DIRECTORY STRUCTURE ---
    /// Cached graph of the project directory structure.
    pub project_dir_cache: Vec<FileNode>,

    // --- MIDI ---
    /// The name of the currently connected MIDI input port.
    pub selected_midi_port: Option<String>,
    /// The MIDI input.
    pub midi_in: Option<MidiInput>,
    /// The names of the available MIDI input ports.
    pub midi_in_ports: MidiInputPorts,

    // --- CPAL DEVICE FETCHING ---
    /// The name of the currently selected CPAL output device.
    pub selected_output_device: Option<cpal::DeviceId>,
    /// The CPAL host, used for fetching audio devices.
    pub host: cpal::Host,
    // The default output device.
    pub default_output_device: Option<cpal::Device>,
    // The fetched audio output devices.
    pub output_devices: Vec<cpal::Device>,

    // --- AUDIO CONTEXT ---
    /// The current audio context.
    pub audio_ctx: AudioContext,

    // --- AUDIO FILE IMPORT ---
    /// The last audio file dropped into the editor, along with the position where it was dropped.
    pub last_audio_drop: Option<(PathBuf, egui::Pos2)>,
}

impl EditorUiState {
    pub fn with_audio_ctx(audio_ctx: AudioContext) -> Self {
        EditorUiState {
            audio_ctx,
            ..Default::default()
        }
    }

    /// Set the selected track to the given one, deselecting the note and the node.
    pub fn select_track(&mut self, track_id: TrackID) {
        self.selection = Selection::Track(track_id);
    }

    /// Set the selected region to the given one, deselecting the note and the node.
    pub fn select_region(&mut self, track_id: TrackID, region_id: RegionID) {
        self.selection = Selection::Region(track_id, region_id);
    }

    /// Set the selected note to the given one.
    pub fn select_note(&mut self, track_id: TrackID, region_id: RegionID, note_id: NoteID) {
        self.selection = Selection::Note(track_id, region_id, note_id);
    }

    /// Set the selected node to the given one.
    pub fn select_node(&mut self, track_id: TrackID, node_id: NodeID) {
        self.selection = Selection::Node(track_id, node_id);
    }

    /// Deselects the currently selected content.
    pub fn deselect_all(&mut self) {
        self.selection = Selection::None;
    }

    /// Gets the ticks per pixel in the timeline.
    pub fn timeline_ticks_per_pixel(&self) -> f32 {
        self.audio_ctx.resolution as f32 / self.timeline_state.pixels_per_beat
    }

    /// Gets the ticks per pixel in the piano roll.
    pub fn piano_roll_ticks_per_pixel(&self) -> f32 {
        self.audio_ctx.resolution as f32 / self.piano_roll_state.pixels_per_beat
    }

    /// Sets the modification state to the given one.
    pub fn set_modification(&mut self, modification: Modification) {
        self.modification = modification;
    }
}
