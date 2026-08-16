mod code_editor_state;
mod dialog_state;
mod modification;
mod node_graph_state;
mod panel_layout;
mod piano_roll_state;
mod status_bar_state;
mod timeline_state;
mod toolbar_state;

pub(crate) use code_editor_state::CodeEditorState;
pub(crate) use dialog_state::{AddTrackState, DialogState};
pub(crate) use modification::Modification;
pub(crate) use node_graph_state::NodeGraphState;
pub(crate) use panel_layout::{PanelNode, PanelView, SplitDir};
pub(crate) use piano_roll_state::PianoRollState;
pub(crate) use status_bar_state::StatusBarState;
pub(crate) use timeline_state::TimelineState;
pub(crate) use toolbar_state::ToolbarState;

use crate::{actions::FileNode, core::project_ctx::ProjectContext};
use kadent_engine::{
    data_types::{AudioContext, Ticks},
    graph::node_id::NodeID,
    mixer::TrackID,
    thread::AudioError,
    track::{RegionID, note_track::NoteID},
};
use midir::{MidiInput, MidiInputPorts};
use std::{path::PathBuf, time::Instant};

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
    /// The current playhead position in ticks.
    pub playhead_tick: Ticks,

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

    // --- ERRORS ---
    /// Errors to be shown.
    pub errors: Vec<AudioError>,

    // --- EXPORTING ---
    /// The path to write the currently exported project to.
    pub pending_export_path: Option<PathBuf>,

    // --- AUDIO CONTEXT ---
    /// The current audio context.
    pub audio_ctx: AudioContext,
    /// The current project context.
    pub proj_ctx: ProjectContext,
}

impl EditorUiState {
    pub fn new(audio_ctx: AudioContext, proj_ctx: ProjectContext) -> Self {
        EditorUiState {
            panel_layout: PanelNode::default(),
            toolbar_state: ToolbarState::default(),
            dialog_state: DialogState::default(),
            timeline_state: TimelineState::default(),
            piano_roll_state: PianoRollState::default(),
            node_graph_state: NodeGraphState::default(),
            code_editor_state: CodeEditorState::default(),
            status_bar_state: StatusBarState::default(),
            is_playing: false,
            playhead_tick: Ticks(0),
            last_edit_time: None,
            selection: Selection::None,
            modification: Modification::None,
            project_dir_cache: Vec::new(),
            selected_midi_port: None,
            midi_in: None,
            midi_in_ports: MidiInputPorts::default(),
            selected_output_device: None,
            host: cpal::default_host(),
            default_output_device: None,
            output_devices: Vec::new(),
            errors: Vec::new(),
            pending_export_path: None,
            audio_ctx,
            proj_ctx,
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
