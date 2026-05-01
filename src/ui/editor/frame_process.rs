use crate::{ui::EditorUi, ui_state::toolbar_state::PeakHold};
use ringbuf::traits::Consumer;
use std::sync::atomic::Ordering;

const PEAK_HOLD_TIME: f32 = 0.5;

impl EditorUi {
    pub(super) fn calculate_playhead(&mut self) {
        let playhead_sample = self.thread_handle.playhead.load(Ordering::Acquire);

        // Calculate if the playhead sample has changed
        if self.ui_state.last_playhead != playhead_sample {
            self.ui_state.playhead_beats = self.project.tempo_map.samples_to_beats(playhead_sample);
            self.ui_state.last_playhead = playhead_sample;
        }
    }

    pub(super) fn process_vu_value(&mut self) {
        let channels = self.project.audio_ctx.channels;
        self.ui_state
            .toolbar_state
            .last_vu_value
            .resize(channels, 0.0);
        self.ui_state.toolbar_state.peak_holds.resize(
            channels,
            PeakHold {
                value: 0.0,
                hold_time: std::time::Instant::now(),
            },
        );

        for channel in 0..channels {
            // Fetch the latest VU value for this channel from the audio thread
            if let Some(v) = self.thread_handle.vu_consumer.try_pop() {
                self.ui_state.toolbar_state.last_vu_value[channel] = v;
            };

            // Update the peak hold values
            let current_vu = self.ui_state.toolbar_state.last_vu_value[channel];
            let peak_hold = &mut self.ui_state.toolbar_state.peak_holds[channel];
            if peak_hold.hold_time.elapsed().as_secs_f32() > PEAK_HOLD_TIME {
                peak_hold.value = current_vu;
                peak_hold.hold_time = std::time::Instant::now();
            } else {
                let current_vu = self.ui_state.toolbar_state.last_vu_value[channel];
                if current_vu > peak_hold.value {
                    peak_hold.value = current_vu;
                    peak_hold.hold_time = std::time::Instant::now();
                }
            }
        }
    }
}
