use crate::{
    core::metadata::TrackType,
    ui::editor::actions::EditorAction,
    ui::{
        EditorState,
        editor::{
            StatusHint,
            state::TimelineCoord,
            views::timeline::{TIMELINE_LEFT_PADDING, edit_panel::x_to_ticks},
        },
        theme,
    },
};
use eframe::egui;
use kadent_engine::{
    data_types::Ticks,
    mixer::TrackID,
    timing::{TimeBounds, TimePosition},
    track::RegionID,
};

impl EditorState {
    pub(super) fn track_row(
        &mut self,
        ui: &mut egui::Ui,
        timeline_coord: &TimelineCoord,
        track_id: &TrackID,
        row_rect: egui::Rect,
        content_top: f32,
    ) {
        self.draw_regions(ui, timeline_coord, track_id, row_rect, content_top);
        self.track_row_gestures(ui, timeline_coord, track_id, row_rect);
    }

    fn draw_regions(
        &mut self,
        ui: &mut egui::Ui,
        timeline_coord: &TimelineCoord,
        track_id: &TrackID,
        row_rect: egui::Rect,
        content_top: f32,
    ) {
        // Get the track metadata
        let Some(track_meta) = self.project.meta.get_track(track_id) else {
            return;
        };

        let ppt = timeline_coord.ppt(self.project.data.audio_ctx.resolution);
        let region_ids: Vec<RegionID> = track_meta.regions.keys().copied().collect();

        // Loop through the regions in the track and draw them
        for region_id in region_ids {
            // Get the region metadata
            let Some(region_meta) = self
                .project
                .meta
                .get_track(track_id)
                .and_then(|t| t.regions.get(&region_id))
            else {
                continue;
            };
            let tempo_map = &self.project.data.tempo_map;
            let (region_start, region_end) = region_meta.bounds.tick_range(tempo_map);
            let region_duration = region_end - region_start;

            // Calculate where to put the region (for gesture hit testing)
            let x = row_rect.min.x + TIMELINE_LEFT_PADDING + region_start.0 as f32 * ppt;
            let w = (region_duration.0 as f32 * ppt).max(8.0);
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

            // Handle gestures on the region (dragging and resizing)
            let move_res = self.region_gestures(
                ui,
                timeline_coord,
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
            let Some(track_meta) = self.project.meta.get_track(track_id) else {
                continue;
            };
            let Some(region_meta) = track_meta.regions.get(&region_id) else {
                continue;
            };

            // Calculate the region's position and size based on the updated bounds
            let tempo_map = &self.project.data.tempo_map;
            let (current_start, current_end) = region_meta.bounds.tick_range(tempo_map);
            let current_duration = current_end - current_start;

            let new_region_x =
                row_rect.min.x + TIMELINE_LEFT_PADDING + current_start.0 as f32 * ppt;
            let new_region_width = (current_duration.0 as f32 * ppt).max(8.0);
            let new_region_rect = egui::Rect::from_min_size(
                egui::pos2(new_region_x, row_rect.min.y + 2.0),
                egui::vec2(new_region_width, row_rect.height() - 4.0),
            );
            let draw_rect = new_region_rect.translate(egui::vec2(0.0, y_offset));

            // Draw on the foreground layer unclipped so that the region is not clipped
            let region_painter = if y_offset != 0.0 {
                ui.ctx()
                    .layer_painter(egui::LayerId::new(egui::Order::Foreground, ui.id()))
            } else {
                ui.painter().with_clip_rect(draw_rect)
            };

            // Highlight the stroke if the region is selected
            let stroke = if self.selection.track_and_region_id() == Some((*track_id, region_id)) {
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

            if track_meta.track_type == TrackType::Audio {
                self.draw_waveform_in(ui, *track_id, region_id, &draw_rect);
            } else {
                self.draw_notes_in(ui, track_id, &region_id, &draw_rect);
            }
        }
    }

    fn track_row_gestures(
        &mut self,
        ui: &mut egui::Ui,
        timeline_coord: &TimelineCoord,
        track_id: &TrackID,
        row_rect: egui::Rect,
    ) {
        let response = ui.allocate_rect(row_rect, egui::Sense::click());

        if response.double_clicked() {
            let start = response
                .interact_pointer_pos()
                .map(|pos| {
                    x_to_ticks(
                        timeline_coord,
                        self.project.data.audio_ctx.resolution,
                        pos.x,
                        row_rect,
                    )
                })
                .unwrap_or_default();
            let duration = Ticks(1);
            let bounds = TimeBounds::Musical { start, duration };

            let track_type = self.project.meta.get_track(track_id).map(|m| m.track_type);
            match track_type {
                Some(TrackType::Audio) => {
                    self.push_action(EditorAction::AddAudioRegion(
                        *track_id,
                        "Region".to_string(),
                        bounds,
                    ));
                }
                Some(TrackType::Note) => {
                    self.push_action(EditorAction::AddNoteRegion(
                        *track_id,
                        "Region".to_string(),
                        bounds,
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
        timeline_coord: &TimelineCoord,
        track_id: &TrackID,
        region_id: &RegionID,
        region_rect: egui::Rect,
        resize_rect: egui::Rect,
        content_top: f32,
    ) -> egui::Response {
        let resolution = self.project.data.audio_ctx.resolution;
        let tpp = timeline_coord.tpp(resolution);

        // Get gestures on the region
        let move_res = ui.allocate_rect(region_rect, egui::Sense::drag());
        let resize_res = ui.allocate_rect(resize_rect, egui::Sense::drag());

        if resize_res.hovered() {
            ui.set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
        }

        // Support resize
        if resize_res.dragged() {
            // Select the region
            self.selection.select_region(*track_id, *region_id);
            self.push_action(EditorAction::ArmTrack(*track_id));

            // Calculate the new duration from the drag amount
            let delta_ticks = Ticks((resize_res.drag_delta().x * tpp) as i64);
            if let Some(region) = self
                .project
                .meta
                .get_track_mut(track_id)
                .and_then(|track| track.get_region_mut(region_id))
            {
                let (region_start, region_end) =
                    region.bounds.tick_range(&self.project.data.tempo_map);
                let region_duration = region_end - region_start;

                let new_duration = (region_duration + delta_ticks).max(Ticks::ZERO);
                region.bounds = TimeBounds::Musical {
                    start: region_start,
                    duration: new_duration,
                };

                self.views
                    .status_bar
                    .set_status_hint(StatusHint::RegionRange(region_start, new_duration));
            }
            return move_res;
        } else if resize_res.drag_stopped()
            && let Some(new_duration) = self
                .project
                .meta
                .get_track(track_id)
                .and_then(|track| track.get_region(region_id))
                .map(|region| region.bounds.duration_ticks(&self.project.data.tempo_map))
        {
            self.push_action(EditorAction::SetRegionDuration(
                *track_id,
                *region_id,
                TimePosition::Musical(new_duration),
            ));
            return move_res;
        }

        // Drag to move
        if move_res.dragged() {
            // Select the region
            self.selection.select_region(*track_id, *region_id);
            self.push_action(EditorAction::ArmTrack(*track_id));

            let delta_ticks = Ticks((move_res.drag_delta().x * tpp) as i64);
            if let Some(region) = self
                .project
                .meta
                .get_track_mut(track_id)
                .and_then(|track| track.get_region_mut(region_id))
            {
                let (region_start, region_end) =
                    region.bounds.tick_range(&self.project.data.tempo_map);
                let region_duration = region_end - region_start;

                let new_start = (region_start + delta_ticks).max(Ticks::ZERO);
                region.bounds = TimeBounds::Musical {
                    start: new_start,
                    duration: region_duration,
                };

                self.views
                    .status_bar
                    .set_status_hint(StatusHint::RegionRange(new_start, region_duration));
            }
        } else if move_res.drag_stopped()
            && let Some(new_start) = self
                .project
                .meta
                .get_track_mut(track_id)
                .and_then(|track| track.get_region_mut(region_id))
                .map(|region| region.bounds.start_tick(&self.project.data.tempo_map))
        {
            // Determine which track row the pointer was over when the drag ended,
            // falling back to the original track if it was released outside any row
            let new_track_id = move_res
                .interact_pointer_pos()
                .and_then(|pos| self.y_to_track_id(timeline_coord, pos.y, content_top))
                .unwrap_or(*track_id);

            self.push_action(EditorAction::MoveRegion(
                *track_id,
                *region_id,
                new_track_id,
                TimePosition::Musical(new_start),
            ));
        }

        move_res
    }

    /// Converts a screen-space y position to the track it falls into,
    /// given the y position of the top of the track list content area.
    fn y_to_track_id(
        &self,
        timeline_coord: &TimelineCoord,
        y: f32,
        content_top: f32,
    ) -> Option<TrackID> {
        let track_height = timeline_coord.y_zoom;
        if y < content_top || track_height <= 0.0 {
            return None;
        }
        let index = ((y - content_top) / track_height) as usize;
        self.project.meta.track_order.get(index).copied()
    }
}
