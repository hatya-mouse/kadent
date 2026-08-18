use crate::{
    core::metadata::{GraphMeta, TrackMeta, TrackType},
    ui::EditorState,
};
use eframe::egui;
use kadent_engine::{
    mixer::TrackID,
    track::{Track, audio_track::AudioTrack, note_track::NoteTrack},
};

impl EditorState {
    /// Adds a new track to the project and the project metadata.
    pub(crate) fn add_track(
        &mut self,
        track_type: TrackType,
        name: String,
        color: egui::Color32,
    ) -> TrackID {
        // Create a track with the given type
        let track: Box<dyn Track> = match track_type {
            TrackType::Audio => Box::new(AudioTrack::new()),
            TrackType::Note => Box::new(NoteTrack::new()),
        };
        // Add a track to the project
        let track_id = self.project.data.add_track(track);

        // Register the metadata, initializing the graph meta from the engine track's graph
        // so the input/output nodes created by the track constructor are visible in the UI.
        let mut track_meta = TrackMeta::new(name, color, track_type);
        if let Some(track) = self.project.data.get_track(&track_id) {
            track_meta.graph = GraphMeta::from_graph(track.get_graph());
        }
        self.project.meta.add_track(track_id, track_meta);

        // Update the project on the audio thread
        self.modified_project();

        track_id
    }

    /// Removes a track from the project and the project metadata.
    pub(super) fn remove_track(&mut self, track_id: &TrackID) {
        // Remove the track from the project
        self.project.data.remove_track(track_id);
        // Remove the track metadata
        self.project.meta.remove_track(track_id);

        self.selection.clear();

        // Update the project on the audio thread
        self.modified_project();
    }
}
