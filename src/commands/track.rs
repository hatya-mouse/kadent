use crate::{
    core::metadata::{GraphMeta, TrackMeta, TrackType},
    ui::workspaces::EditorUi,
};
use eframe::egui;
use kadent_engine::{
    mixer::TrackID,
    track::{Track, audio_track::AudioTrack, note_track::NoteTrack},
};

impl EditorUi {
    /// Adds a new track to the project and the project metadata.
    pub(super) fn add_track(&mut self, track_type: TrackType, name: String, color: egui::Color32) {
        // Create a track with the given type
        let track: Box<dyn Track> = match track_type {
            TrackType::Audio => Box::new(AudioTrack::new(self.ui_state.audio_ctx.clone())),
            TrackType::Note => Box::new(NoteTrack::new(self.ui_state.audio_ctx.clone())),
        };
        // Add a track to the project
        let track_id = self.proj_ctx.project.add_track(track);

        // Register the metadata, initializing the graph meta from the engine track's graph
        // so the input/output nodes created by the track constructor are visible in the UI.
        let mut track_meta = TrackMeta::new(name, color, track_type);
        if let Some(track) = self.proj_ctx.project.get_track(&track_id) {
            track_meta.graph = GraphMeta::from_graph(track.get_graph());
        }
        self.proj_ctx.project_meta.add_track(track_id, track_meta);

        // Update the project on the audio thread
        self.modified_project();
    }

    /// Removes a track from the project and the project metadata.
    pub(super) fn remove_track(&mut self, track_id: &TrackID) {
        // Remove the track from the project
        self.proj_ctx.project.remove_track(track_id);
        // Remove the track metadata
        self.proj_ctx.project_meta.remove_track(track_id);

        self.ui_state.deselect_all();

        // Update the project on the audio thread
        self.modified_project();
    }
}
