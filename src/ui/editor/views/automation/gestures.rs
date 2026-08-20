use crate::{
    consts::TIMELINE_LEFT_PADDING,
    ui::{
        EditorState,
        editor::{TimelineCoord, actions::EditorAction},
    },
};
use eframe::egui::{self, Response};
use kadent_engine::{
    data_types::Ticks,
    graph::node_id::NodeID,
    mixer::TrackID,
    node::builtin::{AutomationTrack, CurveType, Keyframe},
};

const KEYFRAME_CLICK_SIZE: f32 = 20.0;

pub(super) fn add_keyframe_gesture(
    response: &Response,
    track: &AutomationTrack,
    last_curve_type: &Option<CurveType>,
    id: (TrackID, NodeID),
    timeline_coord: &TimelineCoord,
    scroll_rect: egui::Rect,
    tpp: f32,
) -> Option<EditorAction> {
    // Keyframe add gesture
    if response.double_clicked()
        && let Some(pos) = response.interact_pointer_pos()
    {
        let origin_pos = egui::pos2(
            scroll_rect.min.x + TIMELINE_LEFT_PADDING - timeline_coord.scroll.x,
            scroll_rect.min.y - timeline_coord.scroll.y,
        );
        let tick = Ticks(((pos.x - origin_pos.x) * tpp) as i64).max(Ticks::ZERO);

        match track {
            AutomationTrack::Float { range, .. } => {
                let value = (1.0
                    - (pos.y - origin_pos.y) / (scroll_rect.height() * timeline_coord.y_scale))
                    .clamp(*range.start(), *range.end());
                let keyframe =
                    Keyframe::new(tick, last_curve_type.unwrap_or(CurveType::Linear), value);
                Some(EditorAction::AddFloatKeyframe(id.0, id.1, keyframe))
            }
            AutomationTrack::Int { range, .. } => {
                let value = ((1.0
                    - (pos.y - origin_pos.y) / (scroll_rect.height() * timeline_coord.y_scale))
                    .round() as i32)
                    .clamp(*range.start(), *range.end());
                let keyframe = Keyframe::new(tick, CurveType::Step, value);
                Some(EditorAction::AddIntKeyframe(id.0, id.1, keyframe))
            }
            AutomationTrack::Bool { .. } => {
                let half_height = scroll_rect.height() * timeline_coord.y_scale * 0.5;
                let value = pos.y - origin_pos.y < half_height;
                let keyframe = Keyframe::new(tick, CurveType::Step, value);
                Some(EditorAction::AddBoolKeyframe(id.0, id.1, keyframe))
            }
        }
    } else {
        None
    }
}

impl EditorState {
    pub(super) fn select_keyframe_gesture(
        &mut self,
        response: &Response,
        keyframe_pos: &[(usize, CurveType, egui::Pos2)],
    ) {
        if response.clicked() {
            let Some(hover_pos) = response.hover_pos() else {
                return;
            };

            for (index, _, pos) in keyframe_pos {
                let rect =
                    egui::Rect::from_center_size(*pos, egui::Vec2::splat(KEYFRAME_CLICK_SIZE));
                if rect.contains(hover_pos)
                    && let Some((track_id, node_id)) = self.selection.track_and_node_id()
                {
                    self.selection.select_keyframe(track_id, node_id, *index);
                    return;
                }
            }
        }
    }
}
