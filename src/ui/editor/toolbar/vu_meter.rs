use crate::{components::vu_meter::vu_meter, ui::EditorUi};
use eframe::egui;
use ringbuf::traits::Consumer;

impl EditorUi {
    pub(super) fn vu_meter(&mut self, ui: &mut egui::Ui) {
        vu_meter(
            ui,
            &self.ui_state.toolbar_state.last_vu_value,
            &self.ui_state.toolbar_state.peak_holds,
            egui::vec2(200.0, 28.0),
            4,
        );
    }
}
