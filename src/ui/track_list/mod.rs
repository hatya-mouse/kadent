mod add_track_dialog;

use crate::{app::KnodiqApp, components::icon_button::icon_button};
use eframe::egui;

impl KnodiqApp {
    pub(super) fn track_list(&mut self, ui: &mut egui::Ui) {
        for track_id in &self.project_meta.track_order {
            if let Some(track_meta) = self.project_meta.tracks.get(track_id) {
                ui.horizontal(|ui| {
                    // Draw track color
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(4.0, 32.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, 0.0, track_meta.color);

                    // Name of the track
                    ui.label(&track_meta.name);
                });
            }
        }

        if icon_button(
            ui,
            egui::Image::new(egui::include_image!("../../../assets/icons/plus.svg")),
        )
        .clicked()
        {
            self.ui_state.show_add_track_dialog = true;
        }
    }
}
