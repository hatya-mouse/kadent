use crate::ui::EditorUi;
use kadent_engine::data_types::Ticks;
use ringbuf::traits::Consumer;
use std::{sync::atomic::Ordering, time::Instant};

#[derive(Debug, Clone)]
pub(crate) struct PeakHold {
    pub value: f32,
    pub hold_time: Instant,
}

const PEAK_HOLD_TIME: f32 = 0.5;

impl EditorUi {
    pub fn calculate_playhead(&mut self) {
        self.ui_state.playhead_tick =
            Ticks(self.thread_handle.playhead_tick.load(Ordering::Acquire));
    }

    pub fn process_vu_value(&mut self) {
        let channels = self.proj_ctx.project_meta.export_ctx.channels;
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
            if peak_hold.hold_time.elapsed().as_secs_f32() > PEAK_HOLD_TIME
                || current_vu > peak_hold.value
            {
                peak_hold.value = current_vu;
                peak_hold.hold_time = std::time::Instant::now();
            }
        }
    }
}
