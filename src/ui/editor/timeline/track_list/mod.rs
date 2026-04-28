mod track_dialog;

use crate::{
    components::icon_button::toolbar_icon_button,
    theme,
    ui::EditorUi,
    ui_state::dialog_state::{AddTrackState, DialogState, TrackType},
};
use eframe::egui;

impl EditorUi {
    pub(super) fn track_list_panel(&mut self, ui: &mut egui::Ui) {
        egui::Frame::new()
            .fill(theme::primary_bg(ui.visuals().dark_mode))
            .show(ui, |ui| {
                for track_id in &self.project_meta.track_order {
                    if let Some(track_meta) = self.project_meta.tracks.get(track_id) {
                        let bg_color = if Some(track_id) == self.ui_state.selected_track.as_ref() {
                            theme::primary_bg(ui.visuals().dark_mode)
                        } else {
                            theme::secondary_bg(ui.visuals().dark_mode)
                        };

                        let track_frame = egui::Frame::new().fill(bg_color).show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.set_min_width(ui.available_width());

                                // Draw track color
                                let (rect, _) = ui.allocate_exact_size(
                                    egui::vec2(4.0, self.ui_state.timeline_state.track_height),
                                    egui::Sense::hover(),
                                );
                                ui.painter().rect_filled(rect, 0.0, track_meta.color);

                                // Name of the track
                                ui.label(
                                    egui::RichText::new(&track_meta.name)
                                        .size(theme::normal_font_size())
                                        .color(theme::primary_fg(ui.visuals().dark_mode)),
                                );
                            });
                        });

                        // Select the track on click
                        let response =
                            ui.allocate_rect(track_frame.response.rect, egui::Sense::click());
                        if response.clicked() {
                            self.ui_state.set_selected_track(*track_id);
                        }
                    }
                }

                if toolbar_icon_button(
                    ui,
                    egui::Image::new(egui::include_image!("../../../../../assets/icons/plus.svg")),
                )
                .clicked()
                {
                    self.ui_state.dialog_state = DialogState::AddTrack(AddTrackState {
                        selected_track_type: TrackType::Audio,
                        name: String::new(),
                    });
                }
            });
    }
}
