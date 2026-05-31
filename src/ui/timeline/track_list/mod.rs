mod track_dialog;

use crate::{
    app::KrenicApp,
    colors,
    components::icon_button::icon_button,
    ui_state::dialog_state::{AddTrackState, DialogState, TrackType},
};
use eframe::egui;

impl KrenicApp {
    pub(super) fn track_list_panel(&mut self, ui: &mut egui::Ui) {
        egui::Frame::new()
            .fill(colors::primary_bg(ui.visuals().dark_mode))
            .show(ui, |ui| {
                for track_id in &self.project_meta.track_order {
                    if let Some(track_meta) = self.project_meta.tracks.get(track_id) {
                        ui.horizontal(|ui| {
                            // Draw track color
                            let (rect, _) = ui.allocate_exact_size(
                                egui::vec2(4.0, self.ui_state.timeline_state.track_height),
                                egui::Sense::hover(),
                            );
                            ui.painter().rect_filled(rect, 0.0, track_meta.color);

                            // Name of the track
                            ui.label(&track_meta.name);
                        });
                    }
                }

                if icon_button(
                    ui,
                    egui::Image::new(egui::include_image!("../../../../assets/icons/plus.svg")),
                )
                .clicked()
                {
                    self.ui_state.dialog_state = DialogState::AddTrack(AddTrackState {
                        selected_track_type: TrackType::AudioTrack,
                        name: String::new(),
                    });
                }
            });
    }
}
