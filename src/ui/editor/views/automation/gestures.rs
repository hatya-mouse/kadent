use crate::core::audio_engine::{
    data_types::Ticks,
    graph::node_id::NodeID,
    mixer::TrackID,
    node::builtin::{AutomationTrack, CurveType, Keyframe},
};
use crate::{
    consts::TIMELINE_LEFT_PADDING,
    ui::{
        EditorState,
        editor::{
            TimelineCoord,
            actions::{EditorAction, KeyframeType},
        },
    },
};
use eframe::egui::{self, Response};

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
            AutomationTrack::Float { .. } => {
                let value =
                    1.0 - (pos.y - origin_pos.y) / (scroll_rect.height() * timeline_coord.y_scale);
                let keyframe =
                    Keyframe::new(tick, last_curve_type.unwrap_or(CurveType::Linear), value);
                Some(EditorAction::AddKeyframe(
                    id.0,
                    id.1,
                    KeyframeType::Float(keyframe),
                ))
            }
            AutomationTrack::Int { .. } => {
                let value = (1.0
                    - (pos.y - origin_pos.y) / (scroll_rect.height() * timeline_coord.y_scale))
                    .round() as i32;
                let keyframe = Keyframe::new(tick, CurveType::Step, value);
                Some(EditorAction::AddKeyframe(
                    id.0,
                    id.1,
                    KeyframeType::Int(keyframe),
                ))
            }
            AutomationTrack::Bool { .. } => {
                let half_height = scroll_rect.height() * timeline_coord.y_scale * 0.5;
                let value = pos.y - origin_pos.y < half_height;
                let keyframe = Keyframe::new(tick, CurveType::Step, value);
                Some(EditorAction::AddKeyframe(
                    id.0,
                    id.1,
                    KeyframeType::Bool(keyframe),
                ))
            }
        }
    } else {
        None
    }
}

pub(super) fn keyframe_click_gesture(
    response: &Response,
    keyframe_pos: &[(usize, CurveType, egui::Pos2)],
    state: &mut EditorState,
) {
    let Some(hover_pos) = response.hover_pos() else {
        return;
    };
    let Some((track_id, node_id)) = state.selection.track_and_node_id() else {
        return;
    };

    for (index, _, pos) in keyframe_pos {
        let rect = egui::Rect::from_center_size(*pos, egui::Vec2::splat(KEYFRAME_CLICK_SIZE));

        // Select the keyframe when clicked
        if response.clicked() && rect.contains(hover_pos) {
            state.selection.select_keyframe(track_id, node_id, *index);
            break;
        }
    }
}
