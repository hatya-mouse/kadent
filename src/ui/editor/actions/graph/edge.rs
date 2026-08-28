use crate::core::audio_engine::graph::{InputKey, OutputKey};
use crate::core::audio_engine::mixer::TrackID;
use crate::ui::editor::EditorUi;

impl EditorUi {
    pub(crate) fn remove_edge(&mut self, track_id: &TrackID, to: &InputKey) {
        if let Some(track) = self.state.project.data.get_track_mut(track_id) {
            track.get_graph_mut().remove_edge(to);
            // Update the project on the audio thread
            self.state.actions.modified_project();
        }
    }

    pub(crate) fn add_edge(&mut self, track_id: &TrackID, from: OutputKey, to: InputKey) {
        if let Some(track) = self.state.project.data.get_track_mut(track_id) {
            if let Err(err) = track.get_graph_mut().add_edge(from, to) {
                eprintln!("Failed to add edge: {:#?}", err);
                return;
            };

            // Update the project on the audio thread
            self.state.actions.modified_project();
        }
    }
}
