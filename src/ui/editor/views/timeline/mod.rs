mod edit_panel;
mod ruler_area;
mod state;
mod track_list;

pub(crate) use state::TimelineState;

use crate::{
    consts::{
        MAX_SIDEBAR_WIDTH, MAX_TRACK_HEIGHT, MIN_SIDEBAR_WIDTH, MIN_TRACK_HEIGHT,
        TIMELINE_LEFT_PADDING,
    },
    ui::{
        EditorState,
        components::{
            panel_header::panel_header,
            splitter::{SPLITTER_WIDTH, Splitter},
        },
        editor::{TimelineCoord, utils::handle_timeline_zoom},
    },
};
use eframe::egui::{self, scroll_area::ScrollBarVisibility};

#[derive(Clone, Debug)]
pub(crate) struct TimelinePanelState {
    pub(crate) follow_playhead: bool,
    pub(crate) track_list_width: f32,
    pub(crate) timeline_coord: TimelineCoord,
}

impl Default for TimelinePanelState {
    fn default() -> Self {
        Self {
            follow_playhead: false,
            track_list_width: 200.0,
            timeline_coord: TimelineCoord::new(100.0, 60.0, egui::Vec2::ZERO),
        }
    }
}

impl TimelineState {
    pub(in crate::ui::editor) fn ui(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut EditorState,
        panel_state: &mut TimelinePanelState,
    ) {
        let panel_width = ui.available_width();
        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

        // While following, keep the playhead centered in the visible track area
        let timeline_coord = &mut panel_state.timeline_coord;
        let visible_width = (panel_width - panel_state.track_list_width).max(0.0);
        if panel_state.follow_playhead && state.transport.is_playing {
            timeline_coord.scroll.x =
                state.follow_playhead_scroll_offset(timeline_coord, visible_width);
        }

        // Clamp the timeline_scroll by zero and the end of the timeline content width
        // so that it never scrolls past the scrollable area
        let timeline_width = state.timeline_content_width(timeline_coord);
        let max_scroll = (timeline_width - visible_width).max(0.0);
        timeline_coord.scroll.x = timeline_coord.scroll.x.clamp(0.0, max_scroll);

        let bar_scroll_x = panel_header(ui, egui::Margin::ZERO, |ui| {
            self.ruler_area(ui, state, panel_state, visible_width, timeline_width)
        })
        .inner;

        egui::CentralPanel::default()
            .frame(egui::Frame::new())
            .show_inside(ui, |ui| {
                let panel_rect = ui.available_rect_before_wrap();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let divider_x = panel_rect.min.x + panel_state.track_list_width;
                        let edit_panel_rect = panel_rect.with_min_x(divider_x + SPLITTER_WIDTH);
                        self.track_list_panel(
                            ui,
                            state,
                            &panel_state.timeline_coord,
                            panel_state.track_list_width,
                        );

                        Splitter::new(&mut panel_state.track_list_width)
                            .with_min(MIN_SIDEBAR_WIDTH)
                            .with_max(MAX_SIDEBAR_WIDTH)
                            .with_height(panel_rect.height().max(0.0))
                            .show(ui);

                        let scroll_res = egui::ScrollArea::horizontal()
                            .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                            .scroll_offset(panel_state.timeline_coord.scroll)
                            .show(ui, |ui| {
                                ui.set_min_height(panel_rect.height().max(0.0));
                                self.track_edit_panel(
                                    ui,
                                    state,
                                    &panel_state.timeline_coord,
                                    timeline_width,
                                    edit_panel_rect,
                                )
                            });
                        let zoom_gesture_res = handle_timeline_zoom(
                            ui,
                            panel_rect.with_min_x(divider_x),
                            &panel_state.timeline_coord,
                            TIMELINE_LEFT_PADDING,
                            MIN_TRACK_HEIGHT,
                            MAX_TRACK_HEIGHT,
                        );
                        panel_state.timeline_coord.apply_scroll(
                            bar_scroll_x,
                            zoom_gesture_res,
                            scroll_res.state.offset,
                        );
                    });
                });
            });
    }
}
