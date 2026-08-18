mod action_dispatcher;
mod audio_device_manager;
mod code_editor_state;
mod dialog_state;
mod midi_device_manager;
mod node_graph_state;
mod panel_layout;
mod piano_roll_state;
mod selection;
mod status_bar_state;
mod timeline_coord;
mod timeline_state;
mod toolbar_state;
mod transport_state;

pub(crate) use code_editor_state::CodeEditorState;
pub(crate) use dialog_state::{AddTrackState, DialogState};
pub(crate) use node_graph_state::NodeGraphState;
pub(crate) use panel_layout::{PanelNode, PanelView, SplitDir};
pub(crate) use piano_roll_state::PianoRollState;
pub(crate) use selection::Selection;
pub(crate) use status_bar_state::{StatusBarState, StatusHint};
pub(crate) use timeline_coord::TimelineCoord;
pub(crate) use timeline_state::TimelineState;
pub(crate) use toolbar_state::ToolbarState;

pub(crate) use action_dispatcher::ActionDispatcher;
pub(crate) use audio_device_manager::AudioDeviceManager;
pub(crate) use midi_device_manager::MidiDeviceManager;
pub(crate) use transport_state::TransportState;

use kadent_engine::thread::AudioError;

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
