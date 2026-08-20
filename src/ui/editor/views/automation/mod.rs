use crate::{
    consts::{PANEL_HEADER_HEIGHT, TIMELINE_LEFT_PADDING},
    ui::{
        EditorState,
        components::{
            centered_text::centered_text,
            ruler::{RulerConfig, ruler_and_scroll_bar},
        },
        editor::{PanelView, TimelineCoord, views::PanelViewState},
        theme,
    },
};
use eframe::egui::{self, scroll_area::ScrollBarVisibility};
use kadent_engine::{
    data_types::Ticks,
    node::builtin::{AutomationNode, CurveType, Keyframe},
};
use uuid::Uuid;

const VERTICAL_PADDING: f32 = 20.0;
const KEYFRAME_SIZE: f32 = 5.0;

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
        let tpp = timeline_coord.tpp(resolution);
        let ppt = 1.0 / tpp;
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

        // Draw the ruler
        let ruler_config = RulerConfig::new(Ticks(0), TIMELINE_LEFT_PADDING, resolution);
        let (new_scroll_x, ruler_res) = ruler_and_scroll_bar(
            ui,
            ruler_rect,
            &timeline_coord,
            &ruler_config,
            timeline_width,
            available_rect.width(),
        );

        // Draw the automation timeline and keyframes
        let scroll_res = egui::ScrollArea::both()
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
            .horizontal_scroll_offset(timeline_coord.scroll.x)
            .vertical_scroll_offset(timeline_coord.scroll.y)
            .show(ui, |ui| {
                ui.set_max_size(scroll_rect.size());
            });

        // Draw keyframes and curve based on the type of automation track
        let track = &mut automation_node.track;
        let painter = ui.painter_at(scroll_rect);

        let start_tick = Ticks((timeline_coord.scroll.x * tpp) as i64);
        let end_tick = start_tick + Ticks((timeline_width * tpp) as i64);
        let visible_range = start_tick..end_tick;
        track.for_each_normalized_around(visible_range, |tick, curve, value| {
            let x = scroll_rect.min.x + tick.0 as f32 * ppt - timeline_coord.scroll.x;
            let y = scroll_rect.min.y
                + scroll_rect.height() * (1.0 - value) * timeline_coord.y_scale
                - timeline_coord.scroll.y;

            match curve {
                CurveType::Step => {
                    painter.rect(
                        egui::Rect::from_center_size(
                            egui::pos2(x, y),
                            egui::Vec2::splat(KEYFRAME_SIZE),
                        ),
                        0.0,
                        theme::keyframe(),
                        theme::keyframe_stroke(ui.visuals().dark_mode),
                        egui::StrokeKind::Middle,
                    );
                }
                CurveType::Linear => {
                    let mut mesh = egui::Mesh::default();
                    let pos = [
                        egui::pos2(x, y - KEYFRAME_SIZE),
                        egui::pos2(x + KEYFRAME_SIZE, y),
                        egui::pos2(x, y + KEYFRAME_SIZE),
                        egui::pos2(x - KEYFRAME_SIZE, y),
                        egui::pos2(x, y - KEYFRAME_SIZE),
                    ];
                    mesh.colored_vertex(pos[0], egui::Color32::RED);
                    mesh.colored_vertex(pos[1], egui::Color32::RED);
                    mesh.colored_vertex(pos[2], egui::Color32::RED);
                    mesh.colored_vertex(pos[3], egui::Color32::RED);
                    mesh.add_triangle(0, 1, 2);
                    mesh.add_triangle(0, 2, 3);
                    painter.add(egui::Shape::mesh(mesh));

                    // Also draw the outline
                    painter.line(pos.to_vec(), theme::keyframe_stroke(ui.visuals().dark_mode));
                }
                CurveType::Smooth { .. } => {
                    painter.circle(
                        egui::pos2(x, y),
                        KEYFRAME_SIZE,
                        theme::keyframe(),
                        theme::keyframe_stroke(ui.visuals().dark_mode),
                    );
                }
            }
        });

        // Handle gestures
        let response = ui.allocate_rect(scroll_rect, egui::Sense::click());
        if response.double_clicked()
            && let Some(pos) = response.interact_pointer_pos()
        {
            let tick = Ticks(((pos.x + timeline_coord.scroll.x) * tpp) as i64);
            let value = 1.0
                - (pos.y + timeline_coord.scroll.y - scroll_rect.min.y)
                    / (scroll_rect.height() * timeline_coord.y_scale);
            let keyframe = Keyframe::new(tick, CurveType::Linear, value);
            track.add_keyframe(keyframe);

            println!(
                "Added keyframe at tick: {}, value: {}, pos.y: {}, scroll_y: {}, min.y: {}, scroll_rect.height(): {}, timeline-coord.y_scale: {}",
                tick.0,
                value,
                pos.y,
                timeline_coord.scroll.y,
                scroll_rect.min.y,
                scroll_rect.height(),
                timeline_coord.y_scale
            );
        }

        // Insert the new scroll position into the timeline_coord
        if let Some(new_scroll_x) = new_scroll_x {
            timeline_coord.scroll.x = new_scroll_x;
        } else {
            timeline_coord.scroll.x = scroll_res.state.offset.x;
        }
        timeline_coord.scroll.y = scroll_res.state.offset.y;
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
