mod draw;

use crate::{
    consts::{PANEL_HEADER_HEIGHT, TIMELINE_LEFT_PADDING},
    ui::{
        EditorState,
        components::{
            centered_text::centered_text,
            panel_header::panel_header,
            ruler::{RulerConfig, ruler_and_scroll_bar},
        },
        editor::{
            PanelView, TimelineCoord,
            utils::handle_timeline_zoom,
            views::{PanelViewState, automation::draw::draw_automation_timeline},
        },
    },
};
use eframe::egui::{self, scroll_area::ScrollBarVisibility};
use kadent_engine::{
    data_types::Ticks,
    node::builtin::{AutomationNode, CurveType, Keyframe},
};
use uuid::Uuid;

const VERTICAL_PADDING: f32 = 20.0;
const STROKE_WIDTH: f32 = 4.0;
const KEYFRAME_SIZE: f32 = 5.0;
const MIN_AUTOMATION_SCALE: f32 = 1.0;
const MAX_AUTOMATION_SCALE: f32 = 10.0;

impl EditorState {
    pub(in crate::ui::editor) fn automation(&mut self, ui: &mut egui::Ui, panel_id: Uuid) {
        let PanelViewState::Automation(mut timeline_coord) = self
            .views
            .get_panel_state_or_insert(panel_id, PanelView::Automation, || {
                PanelViewState::Automation(TimelineCoord::new(
                    80.0,
                    1.0,
                    egui::vec2(TIMELINE_LEFT_PADDING, 0.0),
                ))
            })
            .clone()
        else {
            return;
        };
        let resolution = self.project.data.audio_ctx.resolution;
        let timeline_width = self.timeline_content_width(&timeline_coord);

        // Get the track and the selected automation node
        let (Some(track_id), Some(node_id)) = (self.selection.track_id(), self.selection.node_id())
        else {
            centered_text(ui, "No Automation Node Selected");
            return;
        };
        let Some(automation_node) = self
            .project
            .data
            .get_track_mut(&track_id)
            .and_then(|track| track.get_graph_mut().get_node_mut(&node_id))
            .and_then(|node| node.as_any_mut().downcast_mut::<AutomationNode>())
        else {
            centered_text(ui, "No Automation Node Selected");
            return;
        };

        let available_rect = ui.available_rect_before_wrap();
        let ruler_bottom_y = available_rect.min.y + PANEL_HEADER_HEIGHT;
        let ruler_rect = available_rect.with_max_y(ruler_bottom_y);
        let scroll_rect = available_rect.with_min_y(ruler_bottom_y);
        let max_scroll = (timeline_width - available_rect.width()).max(0.0);
        timeline_coord.scroll.x = timeline_coord.scroll.x.clamp(0.0, max_scroll);

        // Draw the ruler
        let (bar_scroll_x, ruler_res) = panel_header(ui, egui::Margin::ZERO, |ui| {
            let ruler_config = RulerConfig::new(Ticks(0), TIMELINE_LEFT_PADDING, resolution);
            ruler_and_scroll_bar(
                ui,
                ruler_rect,
                &timeline_coord,
                &ruler_config,
                timeline_width,
                available_rect.width(),
            )
        })
        .inner;

        let track = &mut automation_node.track;
        let tpp = timeline_coord.tpp(resolution);

        // Draw the automation timeline and keyframes
        let scroll_res = egui::CentralPanel::default()
            .frame(egui::Frame::new())
            .show_inside(ui, |ui| {
                egui::ScrollArea::both()
                    .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                    .horizontal_scroll_offset(timeline_coord.scroll.x)
                    .vertical_scroll_offset(timeline_coord.scroll.y)
                    .show(ui, |ui| {
                        ui.set_min_width(timeline_width);
                        draw_automation_timeline(
                            ui,
                            &self.selection,
                            track,
                            &timeline_coord,
                            scroll_rect,
                            timeline_width,
                            tpp,
                        );
                    })
            })
            .inner;

        // Handle gestures
        let response = ui.allocate_rect(scroll_rect, egui::Sense::click());
        if response.double_clicked()
            && let Some(pos) = response.interact_pointer_pos()
        {
            let origin_pos = egui::pos2(
                scroll_rect.min.x + TIMELINE_LEFT_PADDING - timeline_coord.scroll.x,
                scroll_rect.min.y + timeline_coord.scroll.y,
            );
            let tick = Ticks(((pos.x - origin_pos.x) * tpp) as i64).max(Ticks::ZERO);
            let value =
                1.0 - (pos.y - origin_pos.y) / (scroll_rect.height() * timeline_coord.y_scale);
            let keyframe = Keyframe::new(tick, CurveType::Linear, value);
            track.add_keyframe(keyframe);
        }

        // Handle the zoom gesture and apply the scroll offset
        let zoom_gesture_output = handle_timeline_zoom(
            ui,
            scroll_rect,
            &timeline_coord,
            TIMELINE_LEFT_PADDING,
            MIN_AUTOMATION_SCALE,
            MAX_AUTOMATION_SCALE,
        );
        timeline_coord.apply_scroll(bar_scroll_x, zoom_gesture_output, scroll_res.state.offset);

        self.views
            .insert_panel_state(panel_id, PanelViewState::Automation(timeline_coord));
        self.apply_ruler_res(&ruler_res);

        // match track {
        //     AutomationTrack::Float { keyframes, .. } => {}
        //     AutomationTrack::Int { keyframes, .. } => {}
        //     AutomationTrack::Bool { keyframes, .. } => {}
        // }
    }
}
