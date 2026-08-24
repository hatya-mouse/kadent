mod actions;
mod background_results;
mod device_fetching;
mod dialog;
mod frame_process;
mod keyboard;
mod panel;
mod preview_notes;
mod state;
mod status_bar;
mod toolbar;
mod utils;
mod views;

pub(crate) use frame_process::PeakHold;
pub(crate) use panel::{PanelNode, PanelVariant, SplitDir};
pub(crate) use state::*;
pub(crate) use status_bar::{StatusBarView, StatusHint};
pub(crate) use views::{
    AutomationState, CodeBuffer, NodeGraphState, PianoRollState, TimelineState, ViewStates,
};

use crate::core::audio_engine::{
    thread::{AudioCommand, AudioThread, AudioThreadHandle},
    timing::TimePosition,
};
use crate::{
    background_thread::spawn_background_thread,
    core::{
        kasl_node::kasl_syntax_set,
        midi_thread::{MidiCommand, spawn_midi_thread},
        project_ctx::ProjectContext,
    },
    ui::editor::actions::EditorAction,
    ui::{
        components::ruler::RulerResponse,
        editor::state::{ActionDispatcher, AudioDeviceManager, MidiDeviceManager, TransportState},
        theme,
    },
};
use cpal::traits::DeviceTrait;
use eframe::egui;
use egui_extras::syntax_highlighting::SyntectSettings;
use std::{
    sync::{Arc, mpsc},
    time::Duration,
};
use syntect::highlighting::ThemeSet;

pub(crate) struct EditorState {
    // --- PROJECT ---
    /// The current project context that stores the document data.
    pub(crate) project: ProjectContext,

    // --- THREAD COMMUNIATION & HANDLES ---
    /// A thread handle to communicate with the audio thread.
    pub(crate) thread_handle: AudioThreadHandle,
    /// A channel to send MIDI commands to the MIDI thread.
    pub(crate) midi_tx: mpsc::Sender<MidiCommand>,

    // --- AUDIO & MIDI DEVICE ---
    /// The audio device manager to store the available audio devices and the selected device.
    pub(crate) audio_device: AudioDeviceManager,
    /// The MIDI device manager to store the available MIDI devices and the selected device.
    pub(crate) midi_device: MidiDeviceManager,

    // --- TRANSPORT STATE ---
    /// The transport state that stores the current playhead position.
    pub(crate) transport: TransportState,

    // --- BACKEND LOGIC ---
    /// Currently selected content.
    pub(crate) selection: Selection,
    /// Action dispatcher that stores the pending actions to be executed at the end of the frame.
    pub(crate) actions: ActionDispatcher,

    // --- UI COMMAND ---
    /// UI command dispatcher that stores the pending UI commands to be executed at the end of the frame.
    pub(crate) ui_commands: UiCommandDispatcher,

    // --- DEBUG MODE ---
    /// Whether the editor is in the debug mode.
    pub(crate) debug_mode: bool,
}

pub(crate) struct EditorUi {
    // --- EDITOR STATE ---
    pub(crate) state: EditorState,

    // --- UI LAYOUT & VIEW STATES ---
    /// Panel layout tree.
    pub(crate) layout: PanelNode,
    /// The states for the each panel views.
    pub(crate) views: ViewStates,
}

impl EditorUi {
    pub(crate) fn new(proj_ctx: ProjectContext) -> EditorUi {
        let (thread_handle, midi_producer) = AudioThread::spawn(proj_ctx.meta.export_ctx.clone());
        let background_handle = spawn_background_thread();
        let midi_tx = spawn_midi_thread(midi_producer);

        let mut editor_ui = EditorUi {
            state: EditorState {
                project: proj_ctx,
                thread_handle,
                midi_tx,
                audio_device: AudioDeviceManager::default(),
                midi_device: MidiDeviceManager::default(),
                transport: TransportState::default(),
                selection: Selection::default(),
                actions: ActionDispatcher::new(background_handle),
                ui_commands: UiCommandDispatcher::default(),
                debug_mode: false,
            },
            layout: PanelNode::default(),
            views: ViewStates::default(),
        };

        // Load the kasl syntax set and create a syntect settings
        editor_ui.views.code_editor.syntect_settings = Some(Arc::new(SyntectSettings {
            ps: kasl_syntax_set(),
            ts: ThemeSet::load_defaults(),
        }));

        // Fetch the avaliable devices first
        editor_ui.state.fetch_devices();
        editor_ui.state.audio_device.selected_output = editor_ui
            .state
            .audio_device
            .default_output
            .as_ref()
            .and_then(|device| device.id().ok());

        // Load the project structure and cache it
        editor_ui
            .state
            .actions
            .push_action(EditorAction::UpdateDirCache);
        editor_ui.state.actions.modified_project();

        // For each audio region, generate the waveforms
        editor_ui.generate_waveforms();

        editor_ui
    }

    pub(crate) fn editor_ui(&mut self, ui: &mut egui::Ui) {
        self.calculate_playhead();
        self.process_vu_value();
        self.update_preview_notes();
        self.state.handle_keyboard(ui);

        egui::Panel::top(ui.id().with("toolbar"))
            .frame(
                egui::Frame::new()
                    .fill(theme::tertiary_bg(ui.visuals().dark_mode))
                    .inner_margin(egui::Margin::symmetric(12, 0)),
            )
            .exact_size(44.0)
            .show_inside(ui, |ui| {
                self.views.toolbar.ui(ui, &mut self.state);
            });

        // The status bar should display the modification state from the last frame
        egui::Panel::bottom(ui.id().with("status_bar"))
            .frame(
                egui::Frame::new()
                    .fill(theme::tertiary_bg(ui.visuals().dark_mode))
                    .inner_margin(egui::Margin::symmetric(12, 0)),
            )
            .exact_size(32.0)
            .show_inside(ui, |ui| {
                self.views.status_bar.ui(ui, &self.state);
            });

        // Reset the modification state from the last frame
        self.views.status_bar.set_status_hint(StatusHint::None);

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(theme::primary_bg(ui.visuals().dark_mode))
                    .inner_margin(0),
            )
            .show_inside(ui, |ui| {
                let rect = ui.available_rect_before_wrap();
                self.render_panels(ui, rect);
            });

        self.views.dialog.dialog(ui, &mut self.state);
        self.state.update_project();

        // Request a repaint to update the playhead and the VU meter
        ui.ctx().request_repaint_after(Duration::from_millis(16));

        // Execute all pending actions
        self.consume_actions();
        self.process_audio_thread_result();
        // Process the result of the background thread
        self.process_background_results();
    }
}

impl EditorState {
    /// Checks if the project has been modified recently and sends an update command to the audio thread if necessary.
    /// Should not be called directly because this is automatically called.
    fn update_project(&mut self) {
        if let Some(t) = self.actions.last_edit_time
            && t.elapsed() > std::time::Duration::from_millis(300)
        {
            self.actions.last_edit_time = None;

            // Clone the project and send it to the audio thread
            let project = self.project.data.clone();
            if let Err(err) = self
                .thread_handle
                .audio_command_tx
                .send(AudioCommand::UpdateProject(Box::new(project)))
            {
                println!("Failed to send project update command: {err}");
            }
        }
    }

    fn apply_ruler_res(&mut self, ruler_res: &RulerResponse) {
        if let Some(target_tick) = ruler_res.seek_to {
            self.transport.playhead_tick = target_tick;

            if ruler_res.drag_ended {
                self.actions
                    .push_action(EditorAction::Seek(TimePosition::Musical(target_tick)));
            }
        }
    }
}
