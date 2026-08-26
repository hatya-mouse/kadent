//! Processes the actions that are executed in the UI thread at the end of the frame.

mod automation;
mod graph;
mod midi;
mod note;
mod project;
mod region;
mod storage;
mod track;
mod transport;

pub(crate) use project::{FileNode, FileNodeKind};
use uuid::Uuid;

use crate::storage::app_state::AppPreferences;
use crate::{background_thread::BackgroundThreadCommand, core::metadata::TrackType};
use crate::{
    core::audio_engine::{
        data_types::Ticks,
        graph::{InputKey, node_id::NodeID},
        mixer::TrackID,
        node::builtin::{AutomationTrackType, CurveType, Keyframe},
        thread::AudioResult,
        timing::{TimeBounds, TimePosition},
        track::{
            RegionID,
            note_track::{Note, NoteID},
        },
    },
    ui::editor::EditorUi,
};
use eframe::egui;
use midir::MidiInputPort;
use std::path::PathBuf;

#[derive(Clone)]
pub(crate) enum AddibleNodes {
    Kasl,
    Automation,
}

impl AddibleNodes {
    pub(crate) fn name(&self) -> &str {
        match self {
            AddibleNodes::Kasl => "KASL Node",
            AddibleNodes::Automation => "Automation Node",
        }
    }

    pub(crate) fn all() -> Vec<AddibleNodes> {
        vec![AddibleNodes::Kasl, AddibleNodes::Automation]
    }
}

#[derive(Clone)]
pub(crate) enum KeyframeType {
    Float(Keyframe<f32>),
    Int(Keyframe<i32>),
    Bool(Keyframe<bool>),
}

#[derive(Clone)]
pub(crate) enum KeyframeValue {
    Float(f32),
    Int(i32),
    Bool(bool),
}

pub(crate) enum EditorAction {
    // --- PROJECT ---
    /// Save the current project and the currently opened programs in the code editor.
    SaveAll,
    /// Open a project from disk.
    /// `(path)`
    OpenProject(PathBuf),
    /// Exports a project to a WAV file.
    /// `(path)`
    ExportProject(PathBuf),
    /// Load a audio file into the given track at the given start position.
    /// `(path, start)`
    ImportAudioFile(PathBuf, Ticks),
    /// Sets the range of the project to be exported.
    /// `(bounds)`
    SetProjectRange(TimeBounds),

    // --- STORAGE ---
    /// Updates the cache of the file tree.
    UpdateDirCache,
    /// Creates a new file at the given path.
    /// `(path)`
    CreateFile(PathBuf),
    /// Creates a new directory at the given path.
    /// `(path)`
    CreateDirectory(PathBuf),
    /// Moves a file to the trash.
    /// `(path)`
    MoveFileToTrash(PathBuf),
    /// Saves the given code buffer to disk.
    /// `(panel_id)`
    SaveCodeBuffer(Uuid),
    /// Closes the file in the code editor without saving it to disk.
    /// `(panel_id)`
    CloseCodeBuffer(Uuid),
    /// Moves the given file or directory to the the given path.
    /// `(from, to)`
    MoveFile(PathBuf, PathBuf),
    /// Opens a file in the code editor, replacing the current buffer.
    /// `(panel_id, path)`
    OpenFileInCodeEditor(Uuid, PathBuf),

    // --- TRANSPORT ---
    /// Start plyaing the project.
    Play,
    /// Pause playing the project.
    Pause,
    /// Seek to the specific position in the project.
    /// `(time)`
    Seek(TimePosition),

    // --- TRACK ---
    /// Add a new track to the project.
    /// `(track_type, name, color)`
    AddTrack(TrackType, String, egui::Color32),
    /// Remove a track from the project.
    /// `(track_id)`
    RemoveTrack(TrackID),

    // --- REGION ---
    /// Add a new audio region to the given audio track.
    /// `(track_id, name, bounds)`
    AddAudioRegion(TrackID, String, TimeBounds),
    /// Add a new note region to the given note track.
    /// `(track_id, name, bounds)`
    AddNoteRegion(TrackID, String, TimeBounds),
    /// Moves the region to the given start time and the track.
    /// `(track_id, region_id, new_track_id, new_start)`
    MoveRegion(TrackID, RegionID, TrackID, TimePosition),
    /// Sets the duration of the region to the given one.
    /// `(track_id, region_id, new_duration)`
    SetRegionDuration(TrackID, RegionID, TimePosition),
    /// Remove a region from the given track.
    /// `(track_id, region_id)`
    RemoveRegion(TrackID, RegionID),

    // --- NODE GRAPH ---
    /// Add a new node to the given track's node graph.
    /// `(track_id, pos)`
    AddNode(TrackID, AddibleNodes, egui::Pos2),
    /// Remove an edge from the given track's node graph.
    /// `(track_id, to)`
    RemoveEdge(TrackID, InputKey),
    /// Add an edge to the given track's node graph.
    /// `(track_id, from: (NodeID, usize), to)`
    AddEdge(TrackID, (NodeID, usize), InputKey),
    /// Compile a KASL program attached to the given node.
    /// `(track_id, node_id)`
    CompileKasl(TrackID, NodeID),
    /// Remove a node from the given track.
    /// `(track_id, node_id)`
    RemoveNode(TrackID, NodeID),

    // --- AUTOMATION ---
    /// Adds a new keyframe to the given automation node.
    /// `(track_id, node_id, keyframe)`
    AddKeyframe(TrackID, NodeID, KeyframeType),
    /// Removes a keyframe from the given automation node.
    /// `(track_id, node_id, keyframe_index)`
    RemoveKeyframe(TrackID, NodeID, usize),
    /// Sets the value of the keyframe at the given index.
    /// `(track_id, node_id, keyframe_index, new_value)`
    SetKeyframeValue(TrackID, NodeID, usize, KeyframeValue),
    /// Sets the keyframe curve type of the keyframe at the given index to the given one.
    /// `(track_id, node_id, keyframe_index, new_curve)`
    SetKeyframeCurveType(TrackID, NodeID, usize, CurveType),
    /// Changes the type of the track for the given automation node.
    /// `(track_id, node_id, new_type)`
    SetAutomationType(TrackID, NodeID, AutomationTrackType),
    /// Sets the maximum value for the given automation node.
    /// `(track_id, node_id, max_value)`
    SetAutomationMaxValue(TrackID, NodeID, KeyframeValue),
    /// Sets the minimum value for the given automation node.
    /// `(track_id, node_id, min_value)`
    SetAutomationMinValue(TrackID, NodeID, KeyframeValue),

    // --- NOTE ---
    /// Add a new note to the given note region.
    /// `(track_id, region_id, start, pitch, duration)`
    AddNote(TrackID, RegionID, Note),
    /// Move a note to a new start position in beats.
    /// `(track_id, region_id, note_id, new_start)`
    MoveNote(TrackID, RegionID, NoteID, Ticks),
    /// Set a pitch of a note in the given note region.
    /// `(track_id, region_id, note_id, new_pitch)`
    SetNotePitch(TrackID, RegionID, NoteID, f32),
    /// Set a duration of a note in the given note region.
    /// `(track_id, region_id, note_id, new_duration)`
    SetNoteDuration(TrackID, RegionID, NoteID, Ticks),
    /// Remove a note from the given note region.
    /// `(track_id, region_id, note_id)`
    RemoveNote(TrackID, RegionID, NoteID),

    // --- MIDI ---
    /// Set the MIDI input port to the given port.
    /// `(midi_in_port)`
    SetMidiInputPort(MidiInputPort),
    /// Disconnect the currently connected MIDI input port.
    DisconnectMidiPort,
    /// Arm the given track.
    /// `(track_id)`
    ArmTrack(TrackID),
    /// Disarm the currently armed track.
    DisarmTrack,
}

impl EditorUi {
    /// Consume all pending actions and execute them in order.
    pub(crate) fn consume_actions(&mut self, preferences: &AppPreferences) {
        let pending_actions: Vec<EditorAction> = self.state.actions.pending.drain(..).collect();
        for action in pending_actions {
            match action {
                // --- PROJECT ---
                EditorAction::SaveAll => {
                    self.save_all();
                }
                EditorAction::OpenProject(path) => {
                    self.open_project(path, preferences);
                }
                EditorAction::ExportProject(path) => {
                    self.export_project(&path);
                }
                EditorAction::ImportAudioFile(path, start) => {
                    self.import_audio_file(&path, start);
                }
                EditorAction::SetProjectRange(bounds) => {
                    self.set_project_range(bounds);
                }

                // --- STORAGE ---
                EditorAction::UpdateDirCache => {
                    self.update_dir_cache();
                }
                EditorAction::CreateFile(path) => {
                    self.create_file(&path);
                }
                EditorAction::CreateDirectory(path) => {
                    self.create_dir(&path);
                }
                EditorAction::MoveFileToTrash(path) => {
                    self.move_file_to_trash(&path);
                }
                EditorAction::SaveCodeBuffer(panel_id) => {
                    self.save_code_buffer(panel_id);
                }
                EditorAction::CloseCodeBuffer(panel_id) => {
                    self.close_code_buffer(panel_id);
                }
                EditorAction::MoveFile(from, to) => {
                    self.move_file(&from, &to);
                }
                EditorAction::OpenFileInCodeEditor(panel_id, path) => {
                    self.open_file_in_code_editor(panel_id, path);
                }

                // --- TRANSPORT ---
                EditorAction::Play => {
                    self.play();
                }
                EditorAction::Pause => {
                    self.pause();
                }
                EditorAction::Seek(time) => {
                    self.seek(time);
                }

                // --- TRACK ---
                EditorAction::AddTrack(track_type, name, color) => {
                    self.add_track(track_type, name, color);
                }
                EditorAction::RemoveTrack(ref track_id) => {
                    self.remove_track(track_id);
                }

                // --- REGION ---
                EditorAction::AddAudioRegion(ref track_id, name, bounds) => {
                    self.add_audio_region(track_id, name, bounds)
                }
                EditorAction::AddNoteRegion(ref track_id, name, bounds) => {
                    self.add_note_region(track_id, name, bounds)
                }
                EditorAction::MoveRegion(
                    ref original_track_id,
                    ref region_id,
                    ref new_track_id,
                    new_start,
                ) => self.move_region(original_track_id, region_id, new_track_id, new_start),
                EditorAction::SetRegionDuration(
                    ref original_track_id,
                    ref region_id,
                    new_duration,
                ) => self.set_region_duration(original_track_id, region_id, new_duration),
                EditorAction::RemoveRegion(ref track_id, ref region_id) => {
                    self.remove_region(track_id, region_id)
                }

                // --- NODE ---
                EditorAction::AddNode(ref track_id, ref node_type, pos) => {
                    self.add_node(track_id, node_type, pos, preferences)
                }
                EditorAction::RemoveEdge(ref track_id, to) => {
                    self.remove_edge(track_id, &to);
                }
                EditorAction::AddEdge(ref track_id, from, to) => {
                    self.add_edge(track_id, from, to);
                }
                EditorAction::CompileKasl(ref track_id, ref node_id) => {
                    self.compile_kasl_node(track_id, node_id)
                }
                EditorAction::RemoveNode(ref track_id, ref node_id) => {
                    self.remove_node(track_id, node_id)
                }

                // --- AUTOMATION ---
                EditorAction::AddKeyframe(ref track_id, ref node_id, keyframe) => {
                    self.add_keyframe(track_id, node_id, keyframe);
                }
                EditorAction::RemoveKeyframe(ref track_id, ref node_id, keyframe_index) => {
                    self.remove_keyframe(track_id, node_id, keyframe_index);
                }
                EditorAction::SetKeyframeValue(
                    ref track_id,
                    ref node_id,
                    keyframe_index,
                    new_value,
                ) => {
                    self.set_keyframe_value(track_id, node_id, keyframe_index, new_value);
                }
                EditorAction::SetKeyframeCurveType(
                    ref track_id,
                    ref node_id,
                    keyframe_index,
                    new_curve,
                ) => {
                    self.set_keyframe_curve_type(track_id, node_id, keyframe_index, new_curve);
                }
                EditorAction::SetAutomationType(ref track_id, ref node_id, new_type) => {
                    self.set_automation_type(track_id, node_id, new_type);
                }
                EditorAction::SetAutomationMaxValue(ref track_id, ref node_id, max_value) => {
                    self.set_automation_max_value(track_id, node_id, max_value);
                }
                EditorAction::SetAutomationMinValue(ref track_id, ref node_id, min_value) => {
                    self.set_automation_min_value(track_id, node_id, min_value);
                }

                // --- NOTE ---
                EditorAction::AddNote(ref track_id, ref region_id, note) => {
                    self.add_note(track_id, region_id, note)
                }
                EditorAction::MoveNote(ref track_id, ref region_id, ref note_id, new_start) => {
                    self.move_note(track_id, region_id, note_id, new_start)
                }
                EditorAction::SetNotePitch(ref track_id, ref region_id, ref note_id, new_pitch) => {
                    self.set_note_pitch(track_id, region_id, note_id, new_pitch)
                }
                EditorAction::SetNoteDuration(
                    ref track_id,
                    ref region_id,
                    ref note_id,
                    new_duration,
                ) => {
                    self.set_note_duration(track_id, region_id, note_id, new_duration);
                }
                EditorAction::RemoveNote(ref track_id, ref region_id, ref note_id) => {
                    self.remove_note(track_id, region_id, note_id)
                }

                // --- MIDI ---
                EditorAction::SetMidiInputPort(midi_in_port) => {
                    self.state.set_midi_input_port(midi_in_port);
                }
                EditorAction::DisconnectMidiPort => {
                    self.state.disconnect_midi_port();
                }
                EditorAction::ArmTrack(track_id) => {
                    self.state.arm_track(track_id);
                }
                EditorAction::DisarmTrack => {
                    self.state.disarm_track();
                }
            }
        }
    }

    /// Handles result returned from the audio thread.
    pub(crate) fn process_audio_thread_result(&mut self) {
        // Wait for the audio thread to generate the samples and send them back
        while let Ok(res) = self.state.thread_handle.result_rx.try_recv() {
            match res {
                Ok(AudioResult::ExportedAudio(samples)) => {
                    let Some(export_path) = self.state.actions.pending_export_path.take() else {
                        return;
                    };

                    self.state
                        .actions
                        .push_background_job(BackgroundThreadCommand::WriteWav {
                            path: export_path,
                            samples,
                            export_ctx: self.state.project.meta.export_ctx.clone(),
                        });
                }
                Err(_) => {
                    // self.pending_export_path.take();
                    // self.ui_state.status_bar_state.show_temp_status("Failed to export project", theme::error_fg());
                }
            }
        }
    }
}
