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
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(8.0, 0.0);

            for track_id in &self.proj_ctx.project_meta.track_order {
                if let Some(track_meta) = self.proj_ctx.project_meta.tracks.get(track_id) {
                    // Change the background color of the selected track based on whether the track is selected
                    let is_selected = Some(track_id) == self.ui_state.selection.track_id().as_ref();
                    let bg_color = if is_selected {
                        theme::secondary_bg(ui.visuals().dark_mode)
                    } else {
                        egui::Color32::TRANSPARENT
                    };

                    let track_frame = egui::Frame::new().fill(bg_color).show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.set_min_width(self.ui_state.timeline_state.track_list_width);

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
            let mut frame = egui::Frame::new().inner_margin(egui::Margin::symmetric(8, 4));

            let desired_width = self.ui_state.timeline_state.track_list_width;
            let button_size = egui::vec2(desired_width, 28.0);

            let response = ui
                .allocate_ui(button_size, |ui| {
                    // Show the background color when hovered
                    let resp = ui.interact(
                        ui.max_rect(),
                        ui.id().with("add_track_button"),
                        egui::Sense::click(),
                    );
                    if resp.hovered() {
                        frame = frame.fill(theme::text_button_hovered(ui.visuals().dark_mode));
                    }

                    frame.show(ui, |ui| {
                        // Subtract the desired width by 16px due to the inner margin of the frame
                        ui.set_min_width((desired_width - 16.0).max(0.0));

                        ui.horizontal_centered(|ui| {
                            ui.spacing_mut().item_spacing = egui::vec2(4.0, 0.0);

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

                    resp
                })
                .inner;

            // Draw separator line for the "Add Track" button
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
