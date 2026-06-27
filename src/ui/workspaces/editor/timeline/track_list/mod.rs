mod track_dialog;

use crate::{
    core::metadata::TrackType,
    ui::{
        theme,
        workspaces::{
            EditorUi,
            editor::state::{AddTrackState, DialogState},
        },
    },
};
use eframe::egui::{self, include_image};

impl EditorUi {
    pub(super) fn track_list_panel(&mut self, ui: &mut egui::Ui) {
        egui::Frame::new()
            .fill(theme::primary_bg(ui.visuals().dark_mode))
            .show(ui, |ui| {
                let list_width = ui.available_width();

                for track_id in &self.proj_ctx.project_meta.track_order {
                    if let Some(track_meta) = self.proj_ctx.project_meta.tracks.get(track_id) {
                        let bg_color =
                            if Some(track_id) == self.ui_state.selection.track_id().as_ref() {
                                theme::secondary_bg(ui.visuals().dark_mode)
                            } else {
                                theme::primary_bg(ui.visuals().dark_mode)
                            };

                        let track_frame = egui::Frame::new().fill(bg_color).show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.set_min_width(list_width);

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
                        ui.painter().line_segment(
                            [response.rect.left_bottom(), response.rect.right_bottom()],
                            theme::border(ui.visuals().dark_mode),
                        );
                        if response.clicked() {
                            self.ui_state.select_track(*track_id);
                        }
                    }
                }

                // "Add Track" button
                let track_frame = egui::Frame::new()
                    .fill(theme::primary_bg(ui.visuals().dark_mode))
                    .inner_margin(egui::Margin::symmetric(8, 2))
                    .show(ui, |ui| {
                        let desired_size = egui::vec2(list_width, 24.0);
                        let layout = egui::Layout::left_to_right(egui::Align::Center);
                        ui.allocate_ui_with_layout(desired_size, layout, |ui| {
                            ui.set_min_width(list_width);

                            ui.add(
                                egui::Image::new(include_image!(
                                    "../../../../../../assets/icons/plus.svg"
                                ))
                                .fit_to_exact_size(egui::vec2(20.0, 20.0))
                                .tint(theme::primary_fg(ui.visuals().dark_mode)),
                            );
                            ui.label(
                                egui::RichText::new("Add Track")
                                    .size(theme::normal_font_size())
                                    .color(theme::primary_fg(ui.visuals().dark_mode)),
                            );
                        });
                    });

                // Draw background and line for the "Add Track" button
                let response = ui.allocate_rect(track_frame.response.rect, egui::Sense::click());
                ui.painter().line_segment(
                    [response.rect.left_bottom(), response.rect.right_bottom()],
                    theme::border(ui.visuals().dark_mode),
                );

                if response.clicked() {
                    self.ui_state.dialog_state = DialogState::AddTrack(AddTrackState {
                        selected_track_type: TrackType::Audio,
                        name: String::new(),
                    });
                }
            });
    }
}
