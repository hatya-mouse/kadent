mod item;

use crate::{core::audio_engine::thread::AudioError, ui::theme};
use eframe::egui;
use item::draw_error_item;

#[derive(Default)]
pub(crate) struct ErrorListView {
    pub errors: Vec<AudioError>,
}

impl ErrorListView {
    pub(in crate::ui::editor) fn ui(&self, ui: &mut egui::Ui) {
        ui.painter().rect_filled(
            ui.available_rect_before_wrap(),
            0.0,
            theme::secondary_bg(ui.visuals().dark_mode),
        );

        egui::ScrollArea::vertical()
            .id_salt("error_list")
            .auto_shrink(false)
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                for error in &self.errors {
                    draw_error_item(ui, error);
                }
            });
    }

    pub(in crate::ui::editor) fn push_error(&mut self, error: AudioError) {
        self.errors.push(error);
    }
}
