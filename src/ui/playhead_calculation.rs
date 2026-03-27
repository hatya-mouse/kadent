use crate::app::KnodiqApp;
use std::sync::atomic::Ordering;

impl KnodiqApp {
    pub(super) fn calculate_playhead(&mut self) {
        let playhead_sample = self.thread_handle.playhead.load(Ordering::Acquire);

        // Calculate if the playhead sample has changed
        if self.ui_state.last_playhead != playhead_sample {
            self.ui_state.playhead_beats = self.project.tempo_map.samples_to_beats(playhead_sample);
            self.ui_state.last_playhead = playhead_sample;
        }
    }
}
