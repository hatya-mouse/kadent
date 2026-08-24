mod file_control;
mod io_control;
mod state;
mod transport_control;

pub(crate) use state::ToolbarState;

use crate::{
    fonts::RichTextExt,
    ui::{
        EditorState,
        components::{toolbar_group::toolbar_group, vu_meter::vu_meter},
        editor::toolbar::{
            file_control::file_control, io_control::io_control,
            transport_control::transport_control,
        },
        theme,
    },
};
use eframe::egui;

impl ToolbarState {
    pub(super) fn toolbar(&mut self, ui: &mut egui::Ui, state: &mut EditorState) {
        ui.horizontal_centered(|ui| {
            ui.spacing_mut().button_padding = egui::vec2(0.0, 0.0);

            // Set the button hover and clicker color
            ui.visuals_mut().widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
            ui.visuals_mut().widgets.inactive.bg_stroke = egui::Stroke::NONE;

            ui.visuals_mut().widgets.hovered.weak_bg_fill = theme::icon_button_hovered();
            ui.visuals_mut().widgets.hovered.bg_stroke = egui::Stroke::NONE;

            ui.visuals_mut().widgets.active.weak_bg_fill = theme::icon_button_active();
            ui.visuals_mut().widgets.active.bg_stroke = egui::Stroke::NONE;

            transport_control(ui, state);
            playhead_beats(ui, state);
            io_control(ui, state);
            file_control(ui, state);
            vu_meter(
                ui,
                &self.last_vu_value,
                &self.peak_holds,
                egui::vec2(200.0, 28.0),
                4,
            );
        });
    }
}

fn playhead_beats(ui: &mut egui::Ui, state: &EditorState) {
    let playhead_beats =
        state.transport.playhead_tick.0 as f32 / state.project.data.audio_ctx.resolution as f32;
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
