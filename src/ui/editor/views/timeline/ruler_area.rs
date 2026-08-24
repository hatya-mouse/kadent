use crate::core::audio_engine::{data_types::Ticks, timing::TimeBounds};
use crate::ui::editor::TimelineState;
use crate::ui::editor::views::TimelinePanelState;
use crate::{
    consts::{PANEL_HEADER_HEIGHT, PANEL_HEADER_MARGIN, SCROLL_BAR_HEIGHT, TIMELINE_LEFT_PADDING},
    ui::editor::actions::EditorAction,
    ui::{
        EditorState,
        components::{
            icon_button::small_icon_button,
            ruler::{RulerConfig, ruler_and_scroll_bar},
        },
        editor::state::TimelineCoord,
        theme,
    },
};
use eframe::egui;

impl TimelineState {
    /// Returns the new scroll position if the user scrolled the timeline, otherwise returns `None`.
    pub(super) fn ruler_area(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut EditorState,
        panel_state: &mut TimelinePanelState,
        visible_width: f32,
        timeline_width: f32,
    ) -> Option<f32> {
        let panel_rect = ui.available_rect_before_wrap();

        let corner_rect = egui::Rect::from_min_size(
            panel_rect.min,
            egui::vec2(panel_state.track_list_width, PANEL_HEADER_HEIGHT),
        );
        follow_playhead_button(ui, corner_rect, &mut panel_state.follow_playhead);

        let area_rect = egui::Rect::from_min_max(
            egui::pos2(
                panel_rect.min.x + panel_state.track_list_width,
                panel_rect.min.y,
            ),
            egui::pos2(panel_rect.max.x, panel_rect.min.y + PANEL_HEADER_HEIGHT),
        );
        let ruler_config = RulerConfig::new(
            Ticks::ZERO,
            TIMELINE_LEFT_PADDING,
            state.project.data.audio_ctx.resolution,
        );
        let (new_scroll_x, ruler_res) = ruler_and_scroll_bar(
            ui,
            area_rect,
            &panel_state.timeline_coord,
            &ruler_config,
            timeline_width,
            visible_width,
        );
        state.apply_ruler_res(&ruler_res);

        // Add draggable project range indicator
        let ruler_rect = area_rect.with_min_y(area_rect.min.y + SCROLL_BAR_HEIGHT);
        self.project_range_indicator(ui, state, &panel_state.timeline_coord, ruler_rect);

        let vertical_separator_rect = egui::Rect::from_min_size(
            egui::pos2(
                panel_rect.min.x + panel_state.track_list_width,
                panel_rect.min.y,
            ),
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
        state: &mut EditorState,
        timeline_coord: &TimelineCoord,
        ruler_screen_rect: egui::Rect,
    ) {
        let ppb = timeline_coord.ppb;
        let ppt = ppb / state.project.data.audio_ctx.resolution as f32;
        let tempo_map = &state.project.data.tempo_map;
        let origin_x = ruler_screen_rect.min.x - timeline_coord.scroll.x + TIMELINE_LEFT_PADDING;

        let (range_start, range_end) = state.project.data.export_range.tick_range(tempo_map);
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
            let mut export_range = state.project.data.export_range.clone();
            export_range.set_duration_ticks(new_duration, tempo_map);
            state
                .actions
                .push_action(EditorAction::SetProjectRange(export_range));
        } else if start_drag_res.dragged() {
            let drag_delta = start_drag_res.drag_delta();
            let ticks_delta = (drag_delta.x / ppt) as i64;

            // Avoid negative start by using saturating_add
            // Increate the duration when the start is reduced, and vice versa
            let new_start = Ticks(range_start.0.saturating_add(ticks_delta).max(0));
            let start_delta = new_start.0 - range_start.0;
            let new_duration = Ticks(range_duration.0.saturating_sub(start_delta).max(0));

            state
                .actions
                .push_action(EditorAction::SetProjectRange(TimeBounds::Musical {
                    start: new_start,
                    duration: new_duration,
                }));
        }

        // Confirm the change when the mouse is released
        if end_drag_res.drag_stopped() || start_drag_res.drag_stopped() {
            state.actions.push_action(EditorAction::SetProjectRange(
                state.project.data.export_range.clone(),
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
