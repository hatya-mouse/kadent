use crate::{
    consts::{PANEL_HEADER_HEIGHT, PANEL_HEADER_MARGIN, SCROLL_BAR_HEIGHT, TIMELINE_LEFT_PADDING},
    ui::editor::actions::EditorAction,
    ui::{
        EditorState,
        components::{
            icon_button::small_icon_button,
            ruler::{RulerConfig, ruler_and_scroll_bar},
        },
        editor::{StatusHint, state::TimelineCoord},
        theme,
    },
};
use eframe::egui;
use kadent_engine::{data_types::Ticks, timing::TimeBounds};

impl EditorState {
    /// Returns the new scroll position if the user scrolled the timeline, otherwise returns `None`.
    pub(super) fn ruler_area(
        &mut self,
        ui: &mut egui::Ui,
        timeline_coord: &TimelineCoord,
        visible_width: f32,
        timeline_width: f32,
        track_list_width: f32,
        follow_playhead: &mut bool,
    ) -> Option<f32> {
        let panel_rect = ui.available_rect_before_wrap();

        let corner_rect = egui::Rect::from_min_size(
            panel_rect.min,
            egui::vec2(track_list_width, PANEL_HEADER_HEIGHT),
        );
        follow_playhead_button(ui, corner_rect, follow_playhead);

        let area_rect = egui::Rect::from_min_max(
            egui::pos2(panel_rect.min.x + track_list_width, panel_rect.min.y),
            egui::pos2(panel_rect.max.x, panel_rect.min.y + PANEL_HEADER_HEIGHT),
        );
        let ruler_config = RulerConfig::new(
            Ticks::ZERO,
            TIMELINE_LEFT_PADDING,
            self.project.data.audio_ctx.resolution,
        );
        let (new_scroll_x, ruler_res) = ruler_and_scroll_bar(
            ui,
            area_rect,
            timeline_coord,
            &ruler_config,
            timeline_width,
            visible_width,
        );
        self.apply_ruler_res(&ruler_res);

        // Add draggable project range indicator
        let ruler_rect = area_rect.with_min_y(area_rect.min.y + SCROLL_BAR_HEIGHT);
        self.project_range_indicator(ui, timeline_coord, ruler_rect, timeline_coord.scroll.x);

        let vertical_separator_rect = egui::Rect::from_min_size(
            egui::pos2(panel_rect.min.x + track_list_width - 1.0, panel_rect.min.y),
            egui::vec2(2.0, PANEL_HEADER_HEIGHT),
        );
        ui.painter().rect_filled(
            vertical_separator_rect,
            0,
            theme::separator(ui.visuals().dark_mode),
        );

        new_scroll_x
    }

    fn project_range_indicator(
        &mut self,
        ui: &mut egui::Ui,
        timeline_coord: &TimelineCoord,
        ruler_screen_rect: egui::Rect,
        scroll_x: f32,
    ) {
        let ppb = timeline_coord.ppb;
        let ppt = ppb / self.project.data.audio_ctx.resolution as f32;
        let tempo_map = &self.project.data.tempo_map;
        let origin_x = ruler_screen_rect.min.x - scroll_x + TIMELINE_LEFT_PADDING;

        let (range_start, range_end) = self.project.meta.export_range.tick_range(tempo_map);
        let range_duration = range_end - range_start;
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

        // Create a painter with the clip rect set to ruler_screen_rect
        let painter = ui.painter().with_clip_rect(ruler_screen_rect);

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
            self.project
                .meta
                .export_range
                .set_duration_ticks(new_duration, tempo_map);

            self.views
                .status_bar
                .set_status_hint(StatusHint::ProjectRange(range_start, new_duration));
        } else if start_drag_res.dragged() {
            let drag_delta = start_drag_res.drag_delta();
            let ticks_delta = (drag_delta.x / ppt) as i64;

            // Avoid negative start by using saturating_add
            // Increate the duration when the start is reduced, and vice versa
            let new_start = Ticks(range_start.0.saturating_add(ticks_delta).max(0));
            let start_delta = new_start.0 - range_start.0;
            let new_duration = Ticks(range_duration.0.saturating_sub(start_delta).max(0));

            self.project.meta.export_range = TimeBounds::Musical {
                start: new_start,
                duration: new_duration,
            };

            self.views
                .status_bar
                .set_status_hint(StatusHint::ProjectRange(new_start, new_duration));
        }

        // Confirm the change when the mouse is released
        if end_drag_res.drag_stopped() || start_drag_res.drag_stopped() {
            self.actions.push_action(EditorAction::SetProjectRange(
                self.project.meta.export_range.clone(),
            ));
        }
    }
}

/// Draws the follow-playhead toggle in the corner cell above the track list, toggling
/// `follow_playhead` when clicked.
fn follow_playhead_button(ui: &mut egui::Ui, corner_rect: egui::Rect, follow_playhead: &mut bool) {
    ui.scope_builder(
        egui::UiBuilder::new().max_rect(corner_rect.shrink2(PANEL_HEADER_MARGIN.right_bottom())),
        |ui| {
            ui.horizontal_centered(|ui| {
                let icon = egui::include_image!("../../../../../assets/icons/crosshair.svg");
                let response = small_icon_button(ui, egui::Image::new(icon));
                if response.clicked() {
                    *follow_playhead = !*follow_playhead;
                }
                if *follow_playhead {
                    ui.painter()
                        .rect_filled(response.rect, 6.0, theme::icon_button_active());
                }
                response.on_hover_text("Follow playhead");
            });
        },
    );
}
