mod file_control;
mod io_control;
mod transport_control;

use crate::{
    fonts::RichTextExt,
    ui::{
        EditorState,
        components::{toolbar_group::toolbar_group, vu_meter::vu_meter},
        theme,
    },
};
use eframe::egui;

impl EditorState {
    pub(super) fn toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            ui.spacing_mut().button_padding = egui::vec2(0.0, 0.0);

            // Set the button hover and clicker color
            ui.visuals_mut().widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
            ui.visuals_mut().widgets.inactive.bg_stroke = egui::Stroke::NONE;

            ui.visuals_mut().widgets.hovered.weak_bg_fill = theme::icon_button_hovered();
            ui.visuals_mut().widgets.hovered.bg_stroke = egui::Stroke::NONE;

            ui.visuals_mut().widgets.active.weak_bg_fill = theme::icon_button_active();
            ui.visuals_mut().widgets.active.bg_stroke = egui::Stroke::NONE;

            self.transport_control(ui);
            self.playhead_beats(ui);
            self.io_control(ui);
            self.file_control(ui);
            vu_meter(
                ui,
                &self.views.toolbar.last_vu_value,
                &self.views.toolbar.peak_holds,
                egui::vec2(200.0, 28.0),
                4,
            );
        });
    }

    fn playhead_beats(&self, ui: &mut egui::Ui) {
        let playhead_beats =
            self.transport.playhead_tick.0 as f32 / self.project.data.audio_ctx.resolution as f32;
        toolbar_group(ui, |ui| {
            ui.add_sized(
                [200.0, 28.0],
                egui::Label::new(
                    egui::RichText::new(format!("{:.3}", playhead_beats))
                        .size(theme::toolbar_beats_font_size())
                        .color(theme::primary_fg(ui.visuals().dark_mode))
                        .bold(),
                ),
            );
        });
    }
}
