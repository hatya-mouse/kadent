//! Command implementations to communicate with the audio engine.

mod export;
mod graph;
mod note;
mod project_updater;
mod region;
mod track;

use eframe::egui;
use kadent_engine::{
    data_types::Beats,
    graph::node_id::NodeID,
    mixer::TrackID,
    track::{
        RegionID,
        note_track::{Note, NoteID},
    },
};
use std::path::PathBuf;

use crate::ui::workspaces::EditorUi;

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
    /// Save the current project to disk.
    SaveProject,
    /// Open a project from disk.
    /// `(path)`
    OpenProject(PathBuf),
    /// Create a new project.
    /// `(path)`
    SaveCode(PathBuf, String),

    // --- REGION ---
    /// Add a new audio region to the given audio track.
    /// `(track_id, name, start)`
    AddAudioRegion(TrackID, String, Beats),
    /// Add a new note region to the given note track.
    /// `(track_id, name, start)`
    AddNoteRegion(TrackID, String, Beats),
    /// Move a region to a new start position in beats.
    /// `(track_id, region_id, new_start)`
    MoveRegion(TrackID, RegionID, Beats),
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
    MoveNote(TrackID, RegionID, NoteID, Beats),
    /// Set a pitch of a note in the given note region.
    /// `(track_id, region_id, note_id, new_pitch)`
    SetNotePitch(TrackID, RegionID, NoteID, f32),
    /// Remove a note from the given note region.
    /// `(track_id, region_id, note_id)`
    RemoveNote(TrackID, RegionID, NoteID),
}

impl EditorUi {
    /// Consume all pending actions and execute them in order.
    pub(crate) fn consume_actions(&mut self) {
        let pending_actions: Vec<EditorAction> = self.pending_actions.drain(..).collect();
        for action in pending_actions {
            match action {
                // EditorAction::SaveProject => self.save_project(),
                // EditorAction::OpenProject(path) => self.open_project(path),
                // EditorAction::SaveCode(path, code) => self.save_code(path, code),
                EditorAction::AddAudioRegion(ref track_id, name, start) => {
                    self.add_audio_region(track_id, name, start)
                }
                EditorAction::AddNoteRegion(ref track_id, name, start) => {
                    self.add_note_region(track_id, name, start)
                }
                EditorAction::MoveRegion(ref track_id, ref region_id, new_start) => {
                    self.move_region(track_id, region_id, new_start)
                }
                EditorAction::RemoveRegion(ref track_id, ref region_id) => {
                    self.remove_region(track_id, region_id)
                }
                EditorAction::AddNode(ref track_id, ref node_type, pos) => {
                    self.add_node(track_id, node_type, pos)
                }
                EditorAction::RemoveEdge(ref track_id, edge) => self.remove_edge(track_id, edge),
                EditorAction::AddEdge(ref track_id, edge) => self.add_edge(track_id, edge),
                EditorAction::CompileKasl(ref track_id, ref node_id) => {
                    self.compile_kasl_node(track_id, node_id)
                }
                EditorAction::RemoveNode(ref track_id, ref node_id) => {
                    self.remove_node(track_id, node_id)
                }
                EditorAction::AddNote(ref track_id, ref region_id, note) => {
                    self.add_note(track_id, region_id, note)
                }
                EditorAction::MoveNote(ref track_id, ref region_id, ref note_id, new_start) => {
                    self.move_note(track_id, region_id, note_id, new_start)
                }
                EditorAction::SetNotePitch(ref track_id, ref region_id, ref note_id, new_pitch) => {
                    self.set_note_pitch(track_id, region_id, note_id, new_pitch)
                }
                EditorAction::RemoveNote(ref track_id, ref region_id, ref note_id) => {
                    self.remove_note(track_id, region_id, note_id)
                }
                _ => (),
            }
        }
    }
}
