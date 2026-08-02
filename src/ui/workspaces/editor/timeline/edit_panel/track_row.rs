use crate::{
    actions::EditorAction,
    core::metadata::TrackType,
    ui::{
        theme,
        workspaces::{
            EditorUi,
            editor::{state::Modification, timeline::TIMELINE_LEFT_PADDING},
        },
    },
};
use eframe::egui;
use kadent_engine::{data_types::Ticks, mixer::TrackID, track::RegionID};

impl EditorUi {
    pub(super) fn track_row(
        &mut self,
        ui: &mut egui::Ui,
        track_id: &TrackID,
        row_rect: egui::Rect,
        content_top: f32,
    ) {
        self.draw_regions(ui, track_id, row_rect, content_top);
        self.track_row_gestures(ui, track_id, row_rect);
    }

    fn draw_regions(
        &mut self,
        ui: &mut egui::Ui,
        track_id: &TrackID,
        row_rect: egui::Rect,
        content_top: f32,
    ) {
        // Get the track metadata
        let Some(track_meta) = self.proj_ctx.project_meta.get_track(track_id) else {
            return;
        };

        let ppt = self.ui_state.timeline_state.pixels_per_beat
            / self.ui_state.audio_ctx.resolution as f32;
        let region_ids: Vec<RegionID> = track_meta.regions.keys().copied().collect();

        // Loop through the regions in the track and draw them
        for region_id in region_ids {
            // Get the region metadata
            let Some(region_meta) = self
                .proj_ctx
                .project_meta
                .get_track(track_id)
                .and_then(|t| t.regions.get(&region_id))
            else {
                continue;
            };

            // Calculate where to put the region
            let x = row_rect.min.x + TIMELINE_LEFT_PADDING + region_meta.start.0 as f32 * ppt;
            let w = (region_meta.duration.0 as f32 * ppt).max(8.0);
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

            let move_res = self.region_gestures(
                ui,
                track_id,
                &region_id,
                region_rect,
                resize_rect,
                content_top,
            );

            // While being dragged, follow the pointer vertically
            let y_offset = if move_res.dragged() {
                ui.input(|i| i.pointer.press_origin())
                    .zip(move_res.interact_pointer_pos())
                    .map(|(origin, cur)| cur.y - origin.y)
                    .unwrap_or(0.0)
            } else {
                0.0
            };

            // Draw the region box and the name
            let Some(track_meta) = self.proj_ctx.project_meta.get_track(track_id) else {
                continue;
            };
            let Some(region_meta) = track_meta.regions.get(&region_id) else {
                continue;
            };

            let draw_rect = region_rect.translate(egui::vec2(0.0, y_offset));

            // Draw on the foreground layer unclipped so that the region is not clipped
            let region_painter = if y_offset != 0.0 {
                ui.ctx()
                    .layer_painter(egui::LayerId::new(egui::Order::Foreground, ui.id()))
            } else {
                ui.painter().with_clip_rect(draw_rect)
            };

            // Highlight the stroke if the region is selected
            let stroke =
                if self.ui_state.selection.track_and_region_id() == Some((*track_id, region_id)) {
                    egui::Stroke::new(2.0, theme::region_selected(ui.visuals().dark_mode))
                } else {
                    theme::border(ui.visuals().dark_mode)
                };

            region_painter.rect(
                draw_rect,
                4.0,
                track_meta.color.gamma_multiply(0.8),
                stroke,
                egui::StrokeKind::Inside,
            );
            region_painter.text(
                egui::pos2(draw_rect.min.x + 4.0, draw_rect.min.y + 2.0),
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
                .map(|pos| self.x_to_ticks(pos.x, row_rect))
                .unwrap_or_default();

            let track_type = self
                .proj_ctx
                .project_meta
                .get_track(track_id)
                .map(|m| m.track_type);
            match track_type {
                Some(TrackType::Audio) => {
                    self.push_action(EditorAction::AddAudioRegion(
                        *track_id,
                        "Region".to_string(),
                        start,
                    ));
                }
                Some(TrackType::Note) => {
                    self.push_action(EditorAction::AddNoteRegion(
                        *track_id,
                        "Region".to_string(),
                        start,
                    ));
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
        content_top: f32,
    ) -> egui::Response {
        // Get gestures on the region
        let move_res = ui.allocate_rect(region_rect, egui::Sense::drag());
        let resize_res = ui.allocate_rect(resize_rect, egui::Sense::drag());

        if resize_res.hovered() {
            ui.set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
        }

        // Support resize
        if resize_res.dragged() {
            // Select the region
            self.ui_state.select_region(*track_id, *region_id);
            self.push_action(EditorAction::ArmTrack(*track_id));

            // Calculate the new duration from the drag amount
            let delta_ticks = Ticks(
                (resize_res.drag_delta().x * self.ui_state.timeline_ticks_per_pixel()) as i64,
            );
            if let Some(region) = self
                .proj_ctx
                .project_meta
                .get_track_mut(track_id)
                .and_then(|track| track.get_region_mut(region_id))
            {
                let new_duration = (region.duration + delta_ticks).max(Ticks(0));
                region.set_duration(new_duration);

                self.ui_state
                    .set_modification(Modification::RegionRange(region.start, new_duration));
            }
            return move_res;
        } else if resize_res.drag_stopped()
            && let Some(new_duration) = self
                .proj_ctx
                .project_meta
                .get_track(track_id)
                .and_then(|track| track.get_region(region_id))
                .map(|region| region.duration)
        {
            self.push_action(EditorAction::SetRegionDuration(
                *track_id,
                *region_id,
                new_duration,
            ));
            return move_res;
        }

        // Drag to move
        if move_res.dragged() {
            // Select the region
            self.ui_state.select_region(*track_id, *region_id);
            self.push_action(EditorAction::ArmTrack(*track_id));

            let delta_ticks =
                Ticks((move_res.drag_delta().x * self.ui_state.timeline_ticks_per_pixel()) as i64);
            if let Some(region) = self
                .proj_ctx
                .project_meta
                .get_track_mut(track_id)
                .and_then(|track| track.get_region_mut(region_id))
            {
                let new_start = (region.start + delta_ticks).max(Ticks(0));
                region.move_region(new_start);

                self.ui_state
                    .set_modification(Modification::RegionRange(new_start, region.duration));
            }
        } else if move_res.drag_stopped()
            && let Some(new_start) = self
                .proj_ctx
                .project_meta
                .get_track_mut(track_id)
                .and_then(|track| track.get_region_mut(region_id))
                .map(|region| region.start)
        {
            // Determine which track row the pointer was over when the drag ended,
            // falling back to the original track if it was released outside any row
            let new_track_id = move_res
                .interact_pointer_pos()
                .and_then(|pos| self.y_to_track_id(pos.y, content_top))
                .unwrap_or(*track_id);

            self.push_action(EditorAction::MoveRegion(
                *track_id,
                *region_id,
                new_track_id,
                new_start,
            ));
        }

        move_res
    }
}
