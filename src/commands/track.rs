use crate::{app::KnodiqApp, metadata::TrackMeta, ui_state::dialog_state::TrackType};
use eframe::egui;
use knodiq_engine::track::{Track, audio_track::AudioTrack, note_track::NoteTrack};

impl KnodiqApp {
    /// Adds a new track to the project and the project metadata.
    pub(crate) fn add_track(&mut self, track_type: TrackType, name: String, color: egui::Color32) {
        // Create a track with the given type
        let track: Box<dyn Track> = match track_type {
            TrackType::AudioTrack => Box::new(AudioTrack::new(self.project.audio_ctx.clone())),
            TrackType::NoteTrack => Box::new(NoteTrack::new(self.project.audio_ctx.clone())),
        };
        // Add a track to the project
        let track_id = self.project.add_track(track);

        // Register the metadata
        let track_meta = TrackMeta::new(name, color);
        self.project_meta.add_track(track_id, track_meta);

        // Update the project on the audio thread
        self.update_project();
    }
}
