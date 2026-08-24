use crate::{
    core::audio_engine::{
        data_types::Ticks,
        mixer::TrackID,
        timing::{TimeBounds, TimePosition},
        track::RegionID,
    },
    ui::editor::TimelineState,
};
use crate::{
    core::metadata::TrackType,
    ui::editor::actions::EditorAction,
    ui::{
        EditorState,
        editor::{
            state::TimelineCoord,
            views::timeline::{TIMELINE_LEFT_PADDING, edit_panel::x_to_ticks},
        },
        theme,
    },
};
use eframe::egui;

const RESIZABLE_WIDTH: f32 = 5.0;

impl TimelineState {
    pub(super) fn track_row(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut EditorState,
        timeline_coord: &TimelineCoord,
        track_id: &TrackID,
        row_rect: egui::Rect,
        edit_panel_rect: egui::Rect,
    ) {
        self.draw_regions(
            ui,
            state,
            timeline_coord,
            track_id,
            row_rect,
            edit_panel_rect,
        );
        self.track_row_gestures(ui, state, timeline_coord, track_id, row_rect);
    }

    fn draw_regions(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut EditorState,
        timeline_coord: &TimelineCoord,
        track_id: &TrackID,
        row_rect: egui::Rect,
        edit_panel_rect: egui::Rect,
    ) {
        // Get the track metadata
        let Some(track_meta) = state.project.meta.get_track(track_id) else {
            return;
        };

        let ppt = timeline_coord.ppt(state.project.data.audio_ctx.resolution);
        let region_ids: Vec<RegionID> = track_meta.regions.keys().copied().collect();

        // Loop through the regions in the track and draw them
        for region_id in region_ids {
            // Get the region metadata
            let Some(region_meta) = state
                .project
                .meta
                .get_track(track_id)
                .and_then(|t| t.regions.get(&region_id))
            else {
                continue;
            };
            let tempo_map = &state.project.data.tempo_map;
            let (region_start, region_end) = region_meta.bounds.tick_range(tempo_map);
            let region_duration = region_end - region_start;

            // Calculate where to put the region (for gesture hit testing)
            let x = row_rect.min.x + TIMELINE_LEFT_PADDING + region_start.0 as f32 * ppt;
            let w = (region_duration.0 as f32 * ppt).max(8.0);
            let region_rect = egui::Rect::from_min_size(
                egui::pos2(x, row_rect.min.y + 2.0),
                egui::vec2(w, row_rect.height() - 4.0),
            );

            // Handle gestures on the region (dragging and resizing)
            let move_rect = region_rect.intersect(edit_panel_rect);
            let move_res = self.handle_move_gesture(
                ui,
                state,
                timeline_coord,
                track_id,
                &region_id,
                move_rect,
            );
            let is_dragged = move_res.dragged();

            // While being dragged, follow the pointer vertically
            let y_offset = if is_dragged {
                ui.input(|i| i.pointer.press_origin())
                    .zip(move_res.interact_pointer_pos())
                    .map(|(origin, cur)| cur.y - origin.y)
                    .unwrap_or(0.0)
            } else {
                0.0
            };
            let dragged_region_rect = region_rect.translate(egui::vec2(0.0, y_offset));

            // Draw the region box and the name
            let Some(track_meta) = state.project.meta.get_track(track_id) else {
                continue;
            };
            let Some(region_meta) = track_meta.regions.get(&region_id) else {
                continue;
            };

            // While being dragged, draw the region on the foreground layer so it appears above other regions
            let region_painter = if is_dragged {
                ui.ctx()
                    .layer_painter(egui::LayerId::new(egui::Order::Foreground, ui.id()))
                    .with_clip_rect(dragged_region_rect)
            } else {
                ui.painter()
                    .with_clip_rect(edit_panel_rect.intersect(dragged_region_rect))
            };

            // Highlight the stroke if the region is selected
            let stroke = if state.selection.track_and_region_id() == Some((*track_id, region_id)) {
                egui::Stroke::new(2.0, theme::region_selected(ui.visuals().dark_mode))
            } else {
                theme::border(ui.visuals().dark_mode)
            };

            region_painter.rect(
                dragged_region_rect,
                0,
                track_meta.color.gamma_multiply(0.8),
                stroke,
                egui::StrokeKind::Inside,
            );
            region_painter.text(
                egui::pos2(
                    dragged_region_rect.min.x + 4.0,
                    dragged_region_rect.min.y + 2.0,
                ),
                egui::Align2::LEFT_TOP,
                &region_meta.name,
                egui::FontId::proportional(11.0),
                theme::region_text(),
            );

            if track_meta.track_type == TrackType::Audio {
                self.draw_waveform_in(ui, state, *track_id, region_id, &dragged_region_rect);
            } else {
                self.draw_notes_in(ui, state, track_id, &region_id, &dragged_region_rect);
            }

            // Finally handle resize gesture
            let resize_rect = region_rect
                .with_min_x(region_rect.max.x - RESIZABLE_WIDTH)
                .intersect(edit_panel_rect);
            self.handle_resize_gesture(
                ui,
                state,
                timeline_coord,
                track_id,
                &region_id,
                resize_rect,
            );
        }
    }

    fn track_row_gestures(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut EditorState,
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
                        state.project.data.audio_ctx.resolution,
                        pos.x,
                        row_rect,
                    )
                })
                .unwrap_or_default();
            let duration = Ticks(1);
            let bounds = TimeBounds::Musical { start, duration };

            let track_type = state.project.meta.get_track(track_id).map(|m| m.track_type);
            match track_type {
                Some(TrackType::Audio) => {
                    state.actions.push_action(EditorAction::AddAudioRegion(
                        *track_id,
                        "Region".to_string(),
                        bounds,
                    ));
                }
                Some(TrackType::Note) => {
                    state.actions.push_action(EditorAction::AddNoteRegion(
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

    fn handle_move_gesture(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut EditorState,
        timeline_coord: &TimelineCoord,
        track_id: &TrackID,
        region_id: &RegionID,
        move_rect: egui::Rect,
    ) -> egui::Response {
        let resolution = state.project.data.audio_ctx.resolution;
        let tpp = timeline_coord.tpp(resolution);
        let move_res = ui.allocate_rect(move_rect, egui::Sense::drag());
        let content_top = ui.available_rect_before_wrap().min.y;

        // Drag to move
        if move_res.dragged() {
            // Select the region
            state.selection.select_region(*track_id, *region_id);
            state.actions.push_action(EditorAction::ArmTrack(*track_id));

            let delta_ticks = Ticks((move_res.drag_delta().x * tpp) as i64);
            if let Some(region) = state
                .project
                .meta
                .get_track_mut(track_id)
                .and_then(|track| track.get_region_mut(region_id))
            {
                let (region_start, region_end) =
                    region.bounds.tick_range(&state.project.data.tempo_map);
                let region_duration = region_end - region_start;

                let new_start = (region_start + delta_ticks).max(Ticks::ZERO);
                region.bounds = TimeBounds::Musical {
                    start: new_start,
                    duration: region_duration,
                };
            }
        } else if move_res.drag_stopped()
            && let Some(new_start) = state
                .project
                .meta
                .get_track_mut(track_id)
                .and_then(|track| track.get_region_mut(region_id))
                .map(|region| region.bounds.start_tick(&state.project.data.tempo_map))
        {
            // Determine which track row the pointer was over when the drag ended,
            // falling back to the original track if it was released outside any row
            let new_track_id = move_res
                .interact_pointer_pos()
                .and_then(|pos| y_to_track_id(state, timeline_coord, pos.y, content_top))
                .unwrap_or(*track_id);

            state.actions.push_action(EditorAction::MoveRegion(
                *track_id,
                *region_id,
                new_track_id,
                TimePosition::Musical(new_start),
            ));
        }

        move_res
    }

    fn handle_resize_gesture(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut EditorState,
        timeline_coord: &TimelineCoord,
        track_id: &TrackID,
        region_id: &RegionID,
        resize_rect: egui::Rect,
    ) {
        let resolution = state.project.data.audio_ctx.resolution;
        let tpp = timeline_coord.tpp(resolution);
        let resize_res = ui.allocate_rect(resize_rect, egui::Sense::drag());

        if resize_res.hovered() {
            ui.set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
        }

        // Support resize
        if resize_res.dragged() {
            // Select the region
            state.selection.select_region(*track_id, *region_id);
            state.actions.push_action(EditorAction::ArmTrack(*track_id));

            // Calculate the new duration from the drag amount
            let delta_ticks = Ticks((resize_res.drag_delta().x * tpp) as i64);
            if let Some(region) = state
                .project
                .meta
                .get_track_mut(track_id)
                .and_then(|track| track.get_region_mut(region_id))
            {
                let (region_start, region_end) =
                    region.bounds.tick_range(&state.project.data.tempo_map);
                let region_duration = region_end - region_start;

                let new_duration = (region_duration + delta_ticks).max(Ticks::ZERO);
                region.bounds = TimeBounds::Musical {
                    start: region_start,
                    duration: new_duration,
                };
            }
        } else if resize_res.drag_stopped()
            && let Some(new_duration) = state
                .project
                .meta
                .get_track(track_id)
                .and_then(|track| track.get_region(region_id))
                .map(|region| region.bounds.duration_ticks(&state.project.data.tempo_map))
        {
            state.actions.push_action(EditorAction::SetRegionDuration(
                *track_id,
                *region_id,
                TimePosition::Musical(new_duration),
            ));
        }
    }
}

/// Converts a screen-space y position to the track it falls into,
/// given the y position of the top of the track list content area.
fn y_to_track_id(
    state: &EditorState,
    timeline_coord: &TimelineCoord,
    y: f32,
    content_top: f32,
) -> Option<TrackID> {
    let track_height = timeline_coord.y_scale;
    if y < content_top || track_height <= 0.0 {
        return None;
    }
    let index = ((y - content_top) / track_height) as usize;
    state.project.meta.track_order.get(index).copied()
}
