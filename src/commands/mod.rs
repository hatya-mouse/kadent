//! Command implementations to communicate with the audio engine.

mod graph;
mod midi;
mod note;
mod project_updater;
mod region;
mod storage;
mod track;
mod transport;

pub(crate) use storage::{FileNode, FileNodeKind};

use crate::{core::metadata::TrackType, ui::workspaces::EditorUi};
use eframe::egui;
use kadent_engine::{
    data_types::Ticks,
    graph::node_id::NodeID,
    mixer::TrackID,
    track::{
        RegionID,
        note_track::{Note, NoteID},
    },
};
use midir::MidiInputPort;
use std::path::PathBuf;

#[derive(Clone)]
pub(crate) enum AddibleNodes {
    Kasl,
}

impl AddibleNodes {
    pub(crate) fn name(&self) -> &str {
        match self {
            AddibleNodes::Kasl => "KASL Node",
        }
    }

    pub(crate) fn all() -> Vec<AddibleNodes> {
        vec![AddibleNodes::Kasl]
    }
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

    // --- STORAGE ---
    /// Updates the cache of the file tree.
    UpdateDirCache,

    // --- TRANSPORT ---
    /// Start plyaing the project.
    Play,
    /// Pause playing the project.
    Pause,
    /// Seek to the specific position in the project.
    /// `(target_ticks)`
    Seek(Ticks),

    // --- TRACK ---
    /// Add a new track to the project.
    /// `(track_type, name, color)`
    AddTrack(TrackType, String, egui::Color32),
    /// Remove a track from the project.
    /// `(track_id)`
    RemoveTrack(TrackID),

    // --- REGION ---
    /// Add a new audio region to the given audio track.
    /// `(track_id, name, start)`
    AddAudioRegion(TrackID, String, Ticks),
    /// Add a new note region to the given note track.
    /// `(track_id, name, start)`
    AddNoteRegion(TrackID, String, Ticks),
    /// Move a region to a new start position in beats.
    /// `(track_id, region_id, new_start)`
    MoveRegion(TrackID, RegionID, Ticks),
    /// Set a duration of the given region.
    /// `(track_id, region_id, new_duration)`
    SetRegionDuration(TrackID, RegionID, Ticks),
    /// Remove a region from the given track.
    /// `(track_id, region_id)`
    RemoveRegion(TrackID, RegionID),

    // --- NODE GRAPH ---
    /// Add a new node to the given track's node graph.
    /// `(track_id, pos)`
    AddNode(TrackID, AddibleNodes, egui::Pos2),
    /// Remove an edge from the given track's node graph.
    /// `(track_id, edge: (NodeID, usize, NodeID, usize))`
    RemoveEdge(TrackID, (NodeID, usize, NodeID, usize)),
    /// Add an edge to the given track's node graph.
    /// `(track_id, edge: (NodeID, usize, NodeID, usize))
    AddEdge(TrackID, (NodeID, usize, NodeID, usize)),
    /// Compile a KASL program attached to the given node.
    /// `(track_id, node_id)`
    CompileKasl(TrackID, NodeID),
    /// Remove a node from the given track.
    /// `(track_id, node_id)`
    RemoveNode(TrackID, NodeID),

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
    pub(crate) fn consume_actions(&mut self) {
        let pending_actions: Vec<EditorAction> = self.pending_actions.drain(..).collect();
        for action in pending_actions {
            match action {
                // --- PROJECT ---
                EditorAction::SaveAll => {
                    self.save_all();
                }
                EditorAction::OpenProject(path) => {
                    self.open_project(path);
                }
                EditorAction::ExportProject(path) => {
                    self.export_project(&path);
                }

                // --- STORAGE ---
                EditorAction::UpdateDirCache => {
                    self.update_dir_cache();
                }

                // --- TRANSPORT ---
                EditorAction::Play => {
                    self.play();
                }
                EditorAction::Pause => {
                    self.pause();
                }
                EditorAction::Seek(target_ticks) => {
                    self.seek(target_ticks);
                }

                // --- TRACK ---
                EditorAction::AddTrack(track_type, name, color) => {
                    self.add_track(track_type, name, color)
                }
                EditorAction::RemoveTrack(ref track_id) => {
                    self.remove_track(track_id);
                }

                // --- REGION ---
                EditorAction::AddAudioRegion(ref track_id, name, start) => {
                    self.add_audio_region(track_id, name, start)
                }
                EditorAction::AddNoteRegion(ref track_id, name, start) => {
                    self.add_note_region(track_id, name, start)
                }
                EditorAction::MoveRegion(ref track_id, ref region_id, new_start) => {
                    self.move_region(track_id, region_id, new_start)
                }
                EditorAction::SetRegionDuration(ref track_id, ref region_id, new_duration) => {
                    self.set_region_duration(track_id, region_id, new_duration)
                }
                EditorAction::RemoveRegion(ref track_id, ref region_id) => {
                    self.remove_region(track_id, region_id)
                }

                // --- NODE ---
                EditorAction::AddNode(ref track_id, ref node_type, pos) => {
                    self.add_node(track_id, node_type, pos)
                }
                EditorAction::RemoveEdge(ref track_id, edge) => {
                    self.remove_edge(track_id, edge);
                }
                EditorAction::AddEdge(ref track_id, edge) => {
                    self.add_edge(track_id, edge);
                }
                EditorAction::CompileKasl(ref track_id, ref node_id) => {
                    self.compile_kasl_node(track_id, node_id)
                }
                EditorAction::RemoveNode(ref track_id, ref node_id) => {
                    self.remove_node(track_id, node_id)
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
                    self.set_midi_input_port(midi_in_port);
                }
                EditorAction::DisconnectMidiPort => {
                    self.disconnect_midi_port();
                }
                EditorAction::ArmTrack(track_id) => {
                    self.arm_track(track_id);
                }
                EditorAction::DisarmTrack => {
                    self.disarm_track();
                }
            }
        }
    }
}
