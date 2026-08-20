use crate::{
    consts::TIMELINE_LEFT_PADDING,
    ui::{
        editor::{
            Selection, TimelineCoord,
            views::automation::{KEYFRAME_SIZE, STROKE_WIDTH},
        },
        theme,
    },
};
use eframe::egui;
use kadent_engine::{
    data_types::Ticks,
    node::builtin::{AutomationTrack, CurveType},
};

pub(super) fn draw_automation_timeline(
    ui: &egui::Ui,
    selection: &Selection,
    track: &AutomationTrack,
    timeline_coord: &TimelineCoord,
    scroll_rect: egui::Rect,
    timeline_width: f32,
    tpp: f32,
) {
    let ppt = 1.0 / tpp;

    // Calculate the visible range of the keyframes and get normalized keyframe values based on it
    let start_tick = Ticks((timeline_coord.scroll.x - TIMELINE_LEFT_PADDING * tpp) as i64);
    let end_tick = start_tick + Ticks((timeline_width * tpp) as i64);
    let visible_range = start_tick..end_tick;

    let normalized = track.normalized_keyframes_around(visible_range);

    // Draw keyframes and curve based on the type of automation track
    let painter = ui.painter_at(scroll_rect);

    let mut keyframe_positions = Vec::with_capacity(normalized.len());
    for keyframe in normalized.into_iter() {
        // Calculate the screen position of the keyframe
        let x = scroll_rect.min.x + TIMELINE_LEFT_PADDING + keyframe.tick.0 as f32 * ppt
            - timeline_coord.scroll.x;
        let y = scroll_rect.min.y
            + scroll_rect.height() * (1.0 - keyframe.value) * timeline_coord.y_scale
            - timeline_coord.scroll.y;
        let pos = egui::pos2(x, y);
        keyframe_positions.push((keyframe.index, keyframe.curve, pos));
    }

    // Draw the curves based on the calculated positions
    for chunk in keyframe_positions.windows(2) {
        let first = chunk[0];
        let second = chunk[1];
        draw_curve(&painter, &second.1, first.2, second.2);
    }

    // Draw the keyframes on the curves
    let selected_index = selection.keyframe_index();
    for (index, curve, pos) in &keyframe_positions {
        let is_selected = selected_index.is_some_and(|selected_index| *index == selected_index);
        draw_keyframe(ui, &painter, is_selected, curve, *pos);
    }

    // Draw the lines for the first and the last keyframe
    if let Some(first) = keyframe_positions.first()
        && first.2.x > scroll_rect.min.x
    {
        painter.hline(
            scroll_rect.min.x..=first.2.x.min(scroll_rect.max.x),
            first.2.y,
            egui::Stroke::new(STROKE_WIDTH, theme::keyframe(&first.1)),
        );
    }

    if let Some(last) = keyframe_positions.last()
        && last.2.x < scroll_rect.max.x
    {
        painter.hline(
            last.2.x.max(scroll_rect.min.x)..=scroll_rect.max.x,
            last.2.y,
            egui::Stroke::new(STROKE_WIDTH, theme::keyframe(&last.1)),
        );
    }
}

fn draw_curve(painter: &egui::Painter, curve: &CurveType, pos1: egui::Pos2, pos2: egui::Pos2) {
    let color = theme::keyframe(curve);
    match curve {
        CurveType::Step => {
            let mid = egui::pos2(pos2.x, pos1.y);
            painter.line(
                vec![pos1, mid, pos2],
                egui::Stroke::new(STROKE_WIDTH, color),
            );
        }
        CurveType::Linear => {
            painter.line_segment([pos1, pos2], egui::Stroke::new(STROKE_WIDTH, color));
        }
        CurveType::Smooth { .. } => {
            painter.line_segment([pos1, pos2], egui::Stroke::new(STROKE_WIDTH, color));
        }
    }
}

fn draw_keyframe(
    ui: &egui::Ui,
    painter: &egui::Painter,
    is_selected: bool,
    curve: &CurveType,
    pos: egui::Pos2,
) {
    let color = theme::keyframe(curve);
    let stroke = if is_selected {
        egui::Stroke::new(2.0, theme::selected_bg())
    } else {
        theme::keyframe_stroke(ui.visuals().dark_mode)
    };
    match curve {
        CurveType::Step => {
            painter.rect(
                egui::Rect::from_center_size(pos, egui::Vec2::splat(KEYFRAME_SIZE)),
                0.0,
                color,
                stroke,
                egui::StrokeKind::Middle,
            );
        }
        CurveType::Linear => {
            let mut mesh = egui::Mesh::default();
            let outline = [
                egui::pos2(pos.x, pos.y - KEYFRAME_SIZE),
                egui::pos2(pos.x + KEYFRAME_SIZE, pos.y),
                egui::pos2(pos.x, pos.y + KEYFRAME_SIZE),
                egui::pos2(pos.x - KEYFRAME_SIZE, pos.y),
                egui::pos2(pos.x, pos.y - KEYFRAME_SIZE),
            ];
            mesh.colored_vertex(outline[0], color);
            mesh.colored_vertex(outline[1], color);
            mesh.colored_vertex(outline[2], color);
            mesh.colored_vertex(outline[3], color);
            mesh.add_triangle(0, 1, 2);
            mesh.add_triangle(0, 2, 3);
            painter.add(egui::Shape::mesh(mesh));

            // Also draw the outline
            painter.line(outline.to_vec(), stroke);
        }
        CurveType::Smooth { .. } => {
            painter.circle(pos, KEYFRAME_SIZE, color, stroke);
        }
    }
}
