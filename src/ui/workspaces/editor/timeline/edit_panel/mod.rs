mod audio_drop;
mod track_row;

use crate::{
    actions::EditorAction,
    ui::{
        theme,
        workspaces::{
            EditorUi,
            editor::{
                state::Modification,
                timeline::{
                    TIMELINE_LEFT_PADDING, TIMELINE_MAX_PPB, TIMELINE_MIN_PPB,
                    TIMELINE_RIGHT_PADDING,
                },
            },
        },
    },
};
use eframe::egui;
use kadent_engine::data_types::Ticks;

impl EditorUi {
    pub(crate) fn track_edit_panel(&mut self, ui: &mut egui::Ui) {
        let track_height = self.ui_state.timeline_state.track_height;

        // Ensure the scroll area extends past the project range end (or last region end)
        let ppt = self.ui_state.timeline_state.pixels_per_beat
            / self.ui_state.audio_ctx.resolution as f32;
        let range_end_ticks =
            self.proj_ctx.project.range_start.0 + self.proj_ctx.project.range_duration.0;
        let last_region_end = self
            .proj_ctx
            .project_meta
            .track_order
            .iter()
            .filter_map(|id| self.proj_ctx.project_meta.get_track(id))
            .flat_map(|t| t.regions.values())
            .map(|r| r.start.0 + r.duration.0)
            .max()
            .unwrap_or(0);
        let content_end_ticks = range_end_ticks.max(last_region_end);
        ui.set_min_width(
            TIMELINE_LEFT_PADDING + content_end_ticks as f32 * ppt + TIMELINE_RIGHT_PADDING,
        );

        let available = ui.available_rect_before_wrap();

        // Draw each tracks
        let track_order = self.proj_ctx.project_meta.track_order.clone();
        for (i, track_id) in track_order.iter().enumerate() {
            let y = available.min.y + i as f32 * track_height;
            let row_rect = egui::Rect::from_min_size(
                egui::pos2(available.min.x, y),
                egui::vec2(available.width(), track_height),
            );

            self.track_row(ui, track_id, row_rect);

            // Draw a separator
            ui.painter().hline(
                egui::Rangef {
                    min: available.min.x,
                    max: available.min.x + available.width(),
                },
                y + track_height,
                theme::border(ui.visuals().dark_mode),
            );
        }

        // Draw the playhead
        self.playhead(ui, available);

        // Handle pinch / zoom gesture for timeline zoooooming
        self.handle_timeline_zoom(ui, available);

        // Handle dragged or dropped file
        self.try_resolve_audio_drop();
        self.show_dragged_hint(ui);
    }

    pub(super) fn beat_ruler(
        &mut self,
        ui: &mut egui::Ui,
        ruler_screen_rect: egui::Rect,
        scroll_x: f32,
    ) {
        let ppb = self.ui_state.timeline_state.pixels_per_beat;
        let ppt = self.ui_state.timeline_state.pixels_per_beat
            / self.ui_state.audio_ctx.resolution as f32;
        let dark_mode = ui.visuals().dark_mode;
        let origin_x = ruler_screen_rect.min.x - scroll_x + TIMELINE_LEFT_PADDING;

        // --- Gesture handling ---
        let (hover_pos, press_origin, primary_pressed, primary_down, primary_released) =
            ui.input(|i| {
                (
                    i.pointer.hover_pos(),
                    i.pointer.press_origin(),
                    i.pointer.primary_pressed(),
                    i.pointer.primary_down(),
                    i.pointer.primary_released(),
                )
            });

        let seek_key = ui.id().with("ruler_seeking").with((
            ruler_screen_rect.min.x as i32,
            ruler_screen_rect.min.y as i32,
        ));
        let seeking: bool = ui.data(|data| data.get_temp(seek_key).unwrap_or(false));

        if primary_pressed
            && let Some(origin) = press_origin
            && ruler_screen_rect.contains(origin)
        {
            ui.data_mut(|data| data.insert_temp(seek_key, true));
        }

        if let Some(pos) = hover_pos
            && ruler_screen_rect.contains(pos)
        {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        if seeking {
            if primary_down && let Some(pos) = hover_pos {
                let ticks = Ticks(((pos.x - origin_x) / ppt) as i64).max(Ticks(0));
                self.ui_state.playhead_ticks = ticks;
            }

            if primary_released {
                if let Some(pos) = hover_pos {
                    let ticks = Ticks(((pos.x - origin_x) / ppt) as i64).max(Ticks(0));
                    self.push_action(EditorAction::Seek(ticks));
                }
                ui.data_mut(|data| data.remove::<bool>(seek_key));
            }

            if !primary_down {
                ui.data_mut(|data| data.remove::<bool>(seek_key));
            }
        }

        // --- Drawing ---
        let painter = ui.painter().with_clip_rect(ruler_screen_rect);

        let raw_interval = (60.0_f32 / ppb).ceil() as i32;
        let beats_per_label = if raw_interval <= 1 {
            1
        } else if raw_interval <= 2 {
            2
        } else if raw_interval <= 4 {
            4
        } else if raw_interval <= 8 {
            8
        } else if raw_interval <= 16 {
            16
        } else {
            ((raw_interval + 31) / 32) * 32
        };

        // Visible beat range
        let left_beat = ((ruler_screen_rect.min.x - origin_x) / ppb).floor() as i32;
        let right_beat = ((ruler_screen_rect.max.x - origin_x) / ppb).ceil() as i32;
        let first_label_beat = (left_beat / beats_per_label) * beats_per_label;

        let tick_color = theme::ruler_tick(dark_mode);
        let text_color = theme::ruler_label(dark_mode);

        // Major ticks and labels
        let mut beat = first_label_beat;
        while beat <= right_beat {
            if beat >= 0 {
                let x = origin_x + beat as f32 * ppb;

                painter.vline(
                    x,
                    egui::Rangef::new(ruler_screen_rect.min.y, ruler_screen_rect.max.y),
                    egui::Stroke::new(1.0, tick_color),
                );

                painter.text(
                    egui::pos2(x + 3.0, ruler_screen_rect.min.y + 3.0),
                    egui::Align2::LEFT_TOP,
                    format!("{}", beat),
                    egui::FontId::proportional(11.0),
                    text_color,
                );
            }
            beat += beats_per_label;
        }

        // Minor ticks between major ticks
        if ppb >= 30.0 && beats_per_label > 1 {
            for sub_beat in left_beat..=right_beat {
                if sub_beat >= 0 && sub_beat % beats_per_label != 0 {
                    let x = origin_x + sub_beat as f32 * ppb;
                    painter.vline(
                        x,
                        egui::Rangef::new(ruler_screen_rect.max.y - 5.0, ruler_screen_rect.max.y),
                        egui::Stroke::new(1.0, tick_color.gamma_multiply(0.5)),
                    );
                }
            }
        }

        // Add draggable project range indicator
        self.project_range_indicator(ui, &painter, ruler_screen_rect, origin_x, ppt);
    }

    fn project_range_indicator(
        &mut self,
        ui: &mut egui::Ui,
        painter: &egui::Painter,
        ruler_screen_rect: egui::Rect,
        origin_x: f32,
        ppt: f32,
    ) {
        let range_start = self.proj_ctx.project_meta.range_start;
        let range_duration = self.proj_ctx.project_meta.range_duration;
        let range_end = range_start + range_duration;
        let start_x = origin_x + range_start.0 as f32 * ppt;
        let end_x = origin_x + range_end.0 as f32 * ppt;

        let range_left_rect = egui::Rect::from_min_max(
            egui::pos2(ruler_screen_rect.min.x, ruler_screen_rect.min.y),
            egui::pos2(start_x, ruler_screen_rect.max.y),
        );
        let range_right_rect = egui::Rect::from_min_max(
            egui::pos2(end_x, ruler_screen_rect.min.y),
            egui::pos2(ruler_screen_rect.max.x, ruler_screen_rect.max.y),
        );

        // Draw the range that are outside the project range
        painter.rect_filled(range_left_rect, 0.0, theme::range_outside_overlay());
        painter.rect_filled(range_right_rect, 0.0, theme::range_outside_overlay());

        // Create a rect in the each side of the project range to detect dragging
        let start_handle_rect = egui::Rect::from_min_max(
            egui::pos2(start_x - 8.0, ruler_screen_rect.min.y),
            egui::pos2(start_x, ruler_screen_rect.max.y),
        );
        let end_handle_rect = egui::Rect::from_min_max(
            egui::pos2(end_x, ruler_screen_rect.min.y),
            egui::pos2(end_x + 8.0, ruler_screen_rect.max.y),
        );

        // Draw the drag handles
        painter.rect_filled(start_handle_rect, 0.0, theme::selected_bg());
        painter.rect_filled(end_handle_rect, 0.0, theme::selected_bg());

        // Expand the actual handle rects to make it easier to drag
        let start_drag_res = ui.allocate_rect(
            start_handle_rect.expand2(egui::vec2(5.0, 0.0)),
            egui::Sense::drag(),
        );
        let end_drag_res = ui.allocate_rect(
            end_handle_rect.expand2(egui::vec2(5.0, 0.0)),
            egui::Sense::drag(),
        );

        // Change cursor icon when hovering over the drag handles
        if start_drag_res.hovered() || end_drag_res.hovered() {
            ui.set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
        }

        // Handle drag gesture
        // Check the end drag first to prioritize the end drag over the start drag
        if end_drag_res.dragged() {
            let drag_delta = end_drag_res.drag_delta();
            let ticks_delta = (drag_delta.x / ppt) as i64;

            // Avoid negative duration by using saturating_sub
            let new_duration = Ticks(range_duration.0.saturating_add(ticks_delta).max(0));
            self.proj_ctx.project_meta.range_duration = new_duration;

            self.ui_state
                .set_modification(Modification::ProjectRange(range_start, new_duration));
        } else if start_drag_res.dragged() {
            let drag_delta = start_drag_res.drag_delta();
            let ticks_delta = (drag_delta.x / ppt) as i64;

            // Avoid negative start by using saturating_add
            // Increate the duration when the start is reduced, and vice versa
            let new_start = Ticks(range_start.0.saturating_add(ticks_delta).max(0));
            let start_delta = new_start.0 - range_start.0;
            let new_duration = Ticks(range_duration.0.saturating_sub(start_delta).max(0));

            self.proj_ctx.project_meta.range_start = new_start;
            self.proj_ctx.project_meta.range_duration = new_duration;

            self.ui_state
                .set_modification(Modification::ProjectRange(new_start, new_duration));
        }

        // Confirm the change when the mouse is released
        if end_drag_res.drag_stopped() || start_drag_res.drag_stopped() {
            self.push_action(EditorAction::SetProjectRange(
                self.proj_ctx.project_meta.range_start,
                self.proj_ctx.project_meta.range_duration,
            ));
        }
    }

    fn playhead(&mut self, ui: &mut egui::Ui, editor_rect: egui::Rect) {
        let playhead_x = self.ui_state.timeline_state.pixels_per_beat
            * (self.ui_state.playhead_ticks.0 as f32 / self.ui_state.audio_ctx.resolution as f32);

        // Create a new painter to draw on the foreground layer
        ui.painter().vline(
            editor_rect.min.x + playhead_x + TIMELINE_LEFT_PADDING,
            egui::Rangef {
                min: editor_rect.min.y,
                max: editor_rect.max.y,
            },
            egui::Stroke::new(2.0, theme::primary_fg(ui.visuals().dark_mode)),
        );
    }

    fn handle_timeline_zoom(&mut self, ui: &mut egui::Ui, editor_rect: egui::Rect) {
        let editor_res = ui.allocate_rect(editor_rect, egui::Sense::hover());
        if editor_res.hovered() {
            let zoom_delta = ui.input(|i| i.zoom_delta());

            if zoom_delta != 1.0 {
                let new_ppb = self.ui_state.timeline_state.pixels_per_beat * zoom_delta;
                self.ui_state.timeline_state.pixels_per_beat =
                    new_ppb.clamp(TIMELINE_MIN_PPB, TIMELINE_MAX_PPB);
            }
        }
    }

    pub(super) fn x_to_ticks(&self, x: f32, row_rect: egui::Rect) -> Ticks {
        Ticks(
            ((x - row_rect.min.x - TIMELINE_LEFT_PADDING)
                * self.ui_state.timeline_ticks_per_pixel()) as i64,
        )
        .max(Ticks(0))
    }
}
