use crate::{theme, ui::EditorUi, ui_state::dialog_state::TrackType};
use eframe::egui;
use kreniq_engine::{data_types::Beats, mixer::TrackID, track::RegionID};

impl EditorUi {
    pub(super) fn track_row(
        &mut self,
        ui: &mut egui::Ui,
        track_id: &TrackID,
        row_rect: egui::Rect,
    ) {
        self.draw_regions(ui, track_id, row_rect);
        self.track_row_gestures(ui, track_id, row_rect);
    }

    fn draw_regions(&mut self, ui: &mut egui::Ui, track_id: &TrackID, row_rect: egui::Rect) {
        // Get the track metadata
        let Some(track_meta) = self.project_meta.get_track(track_id) else {
            return;
        };

        let ppb = self.ui_state.timeline_state.pixels_per_beat;
        let region_ids: Vec<RegionID> = track_meta.regions.keys().copied().collect();

        // Loop through the regions in the track and draw them
        for region_id in region_ids {
            // Get the region metadata
            let Some(region_meta) = self
                .project_meta
                .get_track(track_id)
                .and_then(|t| t.regions.get(&region_id))
            else {
                continue;
            };

            // Calculate where to put the region
            let x = row_rect.min.x + region_meta.start.0 as f32 * ppb;
            let w = (region_meta.duration.0 as f32 * ppb).max(8.0);
            let region_rect = egui::Rect::from_min_size(
                egui::pos2(x, row_rect.min.y + 2.0),
                egui::vec2(w, row_rect.height() - 4.0),
            );

            // Create a rect on the right side of the region to drag and resize the region
            let draggable_width = 5.0;
            let resize_rect = egui::Rect::from_min_size(
                egui::pos2(x + w - draggable_width, row_rect.min.y + 2.0),
                egui::vec2(draggable_width, row_rect.height() - 4.0),
            );

            self.region_gestures(ui, track_id, &region_id, region_rect, resize_rect);

            // Draw the region box and the name
            let Some(track_meta) = self.project_meta.get_track(track_id) else {
                continue;
            };
            let Some(region_meta) = track_meta.regions.get(&region_id) else {
                continue;
            };
            let painter = ui.painter().with_clip_rect(region_rect);

            // Highlight the stroke if the region is selected
            let stroke = if self.ui_state.selected_region == Some((*track_id, region_id)) {
                egui::Stroke::new(2.0, theme::region_selected(ui.visuals().dark_mode))
            } else {
                egui::Stroke::new(1.0, theme::border(ui.visuals().dark_mode))
            };

            painter.rect(
                region_rect,
                4.0,
                track_meta.color.gamma_multiply(0.8),
                stroke,
                egui::StrokeKind::Inside,
            );
            painter.text(
                egui::pos2(region_rect.min.x + 4.0, region_rect.min.y + 2.0),
                egui::Align2::LEFT_TOP,
                &region_meta.name,
                egui::FontId::proportional(11.0),
                theme::region_text(),
            );
        }
    }

    fn track_row_gestures(&mut self, ui: &mut egui::Ui, track_id: &TrackID, row_rect: egui::Rect) {
        let response = ui.allocate_rect(row_rect, egui::Sense::click());

        if response.double_clicked() {
            let start = response
                .interact_pointer_pos()
                .map(|pos| {
                    Beats(
                        ((pos.x - row_rect.min.x) / self.ui_state.timeline_state.pixels_per_beat)
                            as f64,
                    )
                })
                .unwrap_or_default();

            let track_type = self.project_meta.get_track(track_id).map(|m| m.track_type);
            match track_type {
                Some(TrackType::Audio) => {
                    self.add_audio_region(track_id, "Region".to_string(), start);
                }
                Some(TrackType::Note) => {
                    self.add_note_region(track_id, "Region".to_string(), start);
                }
                None => (),
            }

            ui.close();
        }
    }

    fn region_gestures(
        &mut self,
        ui: &mut egui::Ui,
        track_id: &TrackID,
        region_id: &RegionID,
        region_rect: egui::Rect,
        resize_rect: egui::Rect,
    ) {
        // Get gestures on the region
        let move_res = ui.allocate_rect(region_rect, egui::Sense::drag());
        let resize_res = ui.allocate_rect(resize_rect, egui::Sense::drag());

        if resize_res.hovered() {
            ui.set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
        }

        // Support resize
        if resize_res.dragged() {
            // Select the region
            self.ui_state.set_selected_region(*track_id, *region_id);

            // Calculate the new duration from the drag amount
            let delta_beats = Beats(
                (resize_res.drag_delta().x / self.ui_state.timeline_state.pixels_per_beat) as f64,
            );
            if let Some(region) = self
                .project_meta
                .get_track_mut(track_id)
                .and_then(|track| track.get_region_mut(region_id))
            {
                region.set_duration(Beats((region.duration.0 + delta_beats.0).max(0.0)));
            }
        } else if resize_res.drag_stopped()
            && let Some(new_duration) = self
                .project_meta
                .get_track(track_id)
                .and_then(|track| track.get_region(region_id))
                .map(|region| region.duration)
        {
            self.set_region_duration(track_id, region_id, new_duration);
        } else {
            // Drag to move
            if move_res.dragged() {
                // Select the region
                self.ui_state.set_selected_region(*track_id, *region_id);

                let delta_beats = Beats(
                    (move_res.drag_delta().x / self.ui_state.timeline_state.pixels_per_beat) as f64,
                );
                if let Some(region) = self
                    .project_meta
                    .get_track_mut(track_id)
                    .and_then(|track| track.get_region_mut(region_id))
                {
                    region.move_region(Beats((region.start.0 + delta_beats.0).max(0.0)));
                }
            } else if move_res.drag_stopped()
                && let Some(new_start) = self
                    .project_meta
                    .get_track_mut(track_id)
                    .and_then(|track| track.get_region_mut(region_id))
                    .map(|region| region.start)
            {
                self.move_region(track_id, region_id, new_start);
            }
        }
    }
}
