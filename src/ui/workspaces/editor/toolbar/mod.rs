mod file_control;
mod io_control;
mod transport_control;
mod vu_meter;

use crate::{
    fonts::RichTextExt,
    ui::{components::toolbar_group::toolbar_group, theme, workspaces::EditorUi},
};
use eframe::egui;

impl EditorUi {
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
            self.vu_meter(ui);
        });
    }

    fn playhead_beats(&mut self, ui: &mut egui::Ui) {
        toolbar_group(ui, |ui| {
            ui.add_sized(
                [200.0, 28.0],
                egui::Label::new(
                    egui::RichText::new(format!("{:.3}", self.ui_state.playhead_beats.0))
                        .size(theme::toolbar_beats_font_size())
                        .color(theme::primary_fg(ui.visuals().dark_mode))
                        .bold(),
                ),
            );
        });
    }
}
