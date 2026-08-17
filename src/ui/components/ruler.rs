use crate::{
    consts::{SCROLL_BAR_HEIGHT, TIMELINE_LEFT_PADDING},
    ui::{editor::state::TimelineCoord, theme},
};
use eframe::egui;
use kadent_engine::data_types::Ticks;

/// The minimum width of the scroll bar handle.
const MINIMUM_HANDLE_WIDTH: f32 = 12.0;

#[derive(Default)]
pub struct RulerResponse {
    pub drag_ended: bool,
    pub seek_to: Option<Ticks>,
}

/// Draws the ruler and scroll bar for the timeline in the given rect, and returns the new scroll position when the scroll bar returns it.
pub(crate) fn ruler_and_scroll_bar(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    timeline_coord: &TimelineCoord,
    resolution: u64,
    timeline_width: f32,
    visible_width: f32,
) -> (Option<f32>, RulerResponse) {
    // Top half: shows the scroll bar for scrolling the entire timeline horizontally
    // Bottom half: shows the beat rule
    let scroll_bar_bottom_y = rect.min.y + SCROLL_BAR_HEIGHT;
    let scroll_bar_rect = rect.with_max_y(scroll_bar_bottom_y);
    let ruler_rect = rect.with_min_y(scroll_bar_bottom_y);
    let new_scroll_x = scroll_bar(
        ui,
        scroll_bar_rect,
        timeline_coord.scroll.x,
        visible_width,
        timeline_width,
    );
    let ruler_res = beat_ruler(
        ui,
        resolution,
        timeline_coord.ppb,
        ruler_rect,
        timeline_coord.scroll.x,
    );

    (new_scroll_x, ruler_res)
}

/// Draws a scroll bar of the timeline and returns the new scroll position if the scroll bar handle is dragged.
fn scroll_bar(
    ui: &mut egui::Ui,
    scroll_bar_rect: egui::Rect,
    scroll_x: f32,
    visible_width: f32,
    timeline_width: f32,
) -> Option<f32> {
    let dark_mode = ui.visuals().dark_mode;

    // Draw border on the bottom of the scroll bar area
    let x_range = egui::Rangef::new(scroll_bar_rect.min.x, scroll_bar_rect.max.x);
    ui.painter()
        .hline(x_range, scroll_bar_rect.max.y, theme::border(dark_mode));

    // Draw the scroll bar handle
    let inv_timeline = if timeline_width > 0.0 {
        1.0 / timeline_width
    } else {
        0.0
    };
    let visible_ratio = visible_width * inv_timeline;
    let scroll_ratio = scroll_x * inv_timeline;

    let track_width = scroll_bar_rect.width().max(MINIMUM_HANDLE_WIDTH);
    // Calculate the handle width and position based on the visible ratio and scroll ratio
    let handle_width = (visible_ratio * track_width).clamp(MINIMUM_HANDLE_WIDTH, track_width);
    // The maximum left position of the handle to be
    let max_left_x = track_width - handle_width;
    let handle_left_x = scroll_bar_rect.min.x + (scroll_ratio * track_width).clamp(0.0, max_left_x);
    let handle_right_x = handle_left_x + handle_width;

    let handle_rect = egui::Rect::from_min_max(
        egui::pos2(handle_left_x, scroll_bar_rect.min.y),
        egui::pos2(handle_right_x, scroll_bar_rect.max.y),
    );
    ui.painter()
        .rect_filled(handle_rect, 0, theme::scroll_bar_bg(dark_mode));

    // Add draggable area for the scroll bar handle
    let handle_res = ui.allocate_rect(handle_rect, egui::Sense::drag());
    if handle_res.dragged() {
        ui.set_cursor_icon(egui::CursorIcon::Grabbing);

        // Calculate the new scroll position based on the drag delta
        let handle_delta = handle_res.drag_delta().x;
        let new_scroll_x = scroll_x + (handle_delta / scroll_bar_rect.width()) * timeline_width;
        return Some(new_scroll_x);
    } else if handle_res.hovered() {
        ui.set_cursor_icon(egui::CursorIcon::Grab);
    }

    None
}

/// Draws the ruler in the specified rect and returns the new playhead position if the user clicks on the ruler.
fn beat_ruler(
    ui: &mut egui::Ui,
    resolution: u64,
    ppb: f32,
    ruler_rect: egui::Rect,
    scroll_x: f32,
) -> RulerResponse {
    let mut ruler_res = RulerResponse::default();
    let origin_x = ruler_rect.min.x - scroll_x + TIMELINE_LEFT_PADDING;
    let ppt = ppb / resolution as f32;
    let dark_mode = ui.visuals().dark_mode;

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

    let seek_key = ui
        .id()
        .with("ruler_seeking")
        .with((ruler_rect.min.x as i32, ruler_rect.min.y as i32));
    let seeking: bool = ui.data(|data| data.get_temp(seek_key).unwrap_or(false));

    if primary_pressed
        && let Some(origin) = press_origin
        && ruler_rect.contains(origin)
    {
        ui.data_mut(|data| data.insert_temp(seek_key, true));
    }

    if let Some(pos) = hover_pos
        && ruler_rect.contains(pos)
    {
        ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
    }

    if seeking {
        if primary_down && let Some(pos) = hover_pos {
            let ticks = Ticks(((pos.x - origin_x) / ppt) as i64).max(Ticks(0));
            ruler_res.seek_to = Some(ticks);
        }

        if primary_released {
            if let Some(pos) = hover_pos {
                let ticks = Ticks(((pos.x - origin_x) / ppt) as i64).max(Ticks(0));
                ruler_res.seek_to = Some(ticks);
                ruler_res.drag_ended = true;
            }
            ui.data_mut(|data| data.remove::<bool>(seek_key));
        }

        if !primary_down {
            ui.data_mut(|data| data.remove::<bool>(seek_key));
        }
    }

    // --- Drawing ---
    let painter = ui.painter().with_clip_rect(ruler_rect);

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
    let left_beat = ((ruler_rect.min.x - origin_x) / ppb).floor() as i32;
    let right_beat = ((ruler_rect.max.x - origin_x) / ppb).ceil() as i32;
    let first_label_beat = (left_beat / beats_per_label) * beats_per_label;

    let tick_color = theme::border_color(dark_mode);
    let text_color = theme::ruler_label(dark_mode);

    // Major ticks and labels
    let mut beat = first_label_beat;
    while beat <= right_beat {
        if beat >= 0 {
            let x = origin_x + beat as f32 * ppb;

            painter.vline(
                x,
                egui::Rangef::new(ruler_rect.min.y, ruler_rect.max.y),
                egui::Stroke::new(1.0, tick_color),
            );

            painter.text(
                egui::pos2(x + 3.0, ruler_rect.max.y - 2.0),
                egui::Align2::LEFT_BOTTOM,
                format!("{}", beat),
                egui::FontId::proportional(12.0),
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
                    egui::Rangef::new(ruler_rect.max.y - 8.0, ruler_rect.max.y),
                    egui::Stroke::new(1.0, tick_color.gamma_multiply(0.5)),
                );
            }
        }
    }

    ruler_res
}
