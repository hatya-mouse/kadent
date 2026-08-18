mod action_dispatcher;
mod audio_device_manager;
mod dialog_state;
mod midi_device_manager;
mod panel_layout;
mod selection;
mod timeline_coord;
mod transport_state;

pub(crate) use action_dispatcher::ActionDispatcher;
pub(crate) use audio_device_manager::AudioDeviceManager;
pub(crate) use dialog_state::{AddTrackState, DialogState};
pub(crate) use midi_device_manager::MidiDeviceManager;
pub(crate) use panel_layout::{PanelNode, PanelView, SplitDir};
pub(crate) use selection::Selection;
pub(crate) use timeline_coord::TimelineCoord;
pub(crate) use transport_state::TransportState;

use crate::ui::editor::{
    CodeEditorState, NodeGraphState, PianoRollState, StatusBarState, TimelineState,
    toolbar::ToolbarState,
};
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
