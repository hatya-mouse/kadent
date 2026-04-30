use crate::{components::vu_meter::vu_meter, ui::EditorUi};
use eframe::egui;
use ringbuf::traits::Consumer;

impl EditorUi {
    pub(super) fn vu_meter(&mut self, ui: &egui::Ui) {
        let channels = self.project.audio_ctx.channels;
        self.ui_state.last_vu_value.resize(channels, 0.0);
        for channel in 0..channels {
            if let Some(v) = self.thread_handle.vu_consumer.try_pop() {
                self.ui_state.last_vu_value[channel] = v;
            };
            vu_meter(ui, self.ui_state.last_vu_value[channel], 200.0, 28.0, 4.0);
        }
    }
}
