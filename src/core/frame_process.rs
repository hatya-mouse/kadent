use crate::core::audio_engine::data_types::Ticks;
use crate::ui::EditorState;
use ringbuf::traits::Consumer;
use std::{sync::atomic::Ordering, time::Instant};

#[derive(Debug, Clone)]
pub(crate) struct PeakHold {
    pub(crate) value: f32,
    pub(crate) hold_time: Instant,
}

const PEAK_HOLD_TIME: f32 = 0.5;

impl EditorState {
    pub(crate) fn calculate_playhead(&mut self) {
        self.transport.playhead_tick =
            Ticks(self.thread_handle.playhead_tick.load(Ordering::Acquire));
    }

    pub(crate) fn process_vu_value(&mut self) {
        let channels = self.project.meta.export_ctx.channels;
        self.views.toolbar.last_vu_value.resize(channels, 0.0);
        self.views.toolbar.peak_holds.resize(
            channels,
            PeakHold {
                value: 0.0,
                hold_time: std::time::Instant::now(),
            },
        );

        for channel in 0..channels {
            // Fetch the latest VU value for this channel from the audio thread
            if let Some(v) = self.thread_handle.vu_consumer.try_pop() {
                self.views.toolbar.last_vu_value[channel] = v;
            };

            // Update the peak hold values
            let current_vu = self.views.toolbar.last_vu_value[channel];
            let peak_hold = &mut self.views.toolbar.peak_holds[channel];
            if peak_hold.hold_time.elapsed().as_secs_f32() > PEAK_HOLD_TIME
                || current_vu > peak_hold.value
            {
                peak_hold.value = current_vu;
                peak_hold.hold_time = std::time::Instant::now();
            }
        }
    }
}
