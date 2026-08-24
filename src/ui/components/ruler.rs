use crate::core::audio_engine::data_types::Ticks;
use crate::{
    consts::SCROLL_BAR_HEIGHT,
    ui::{editor::TimelineCoord, theme},
};
use eframe::egui;

/// The minimum width of the scroll bar handle.
const MINIMUM_HANDLE_WIDTH: f32 = 12.0;
/// The height of the minor ticks.
const MINOR_TICK_HEIGHT: f32 = 12.0;

#[derive(Default, Clone)]
pub(crate) struct RulerConfig {
    pub(crate) start_tick: Ticks,
    pub(crate) left_padding: f32,
    pub(crate) resolution: u64,
}

impl RulerConfig {
    pub(crate) fn new(start_tick: Ticks, left_padding: f32, resolution: u64) -> Self {
        Self {
            start_tick,
            left_padding,
            resolution,
        }
    }
}

#[derive(Default, Clone)]
pub(crate) struct RulerResponse {
    pub(crate) drag_ended: bool,
    pub(crate) seek_to: Option<Ticks>,
}

/// Draws the ruler and scroll bar for the timeline in the given rect, and returns the new scroll position when the scroll bar returns it, and the ruler response.
pub(crate) fn ruler_and_scroll_bar(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    timeline_coord: &TimelineCoord,
    ruler_config: &RulerConfig,
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
        ruler_config,
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
    config: &RulerConfig,
    ppb: f32,
    ruler_rect: egui::Rect,
    scroll_x: f32,
) -> RulerResponse {
    let mut ruler_res = RulerResponse::default();
    let ppt = ppb / config.resolution as f32;
    let dark_mode = ui.visuals().dark_mode;

    // Calculate the origin x position of the ruler based on the scroll position, start tick and left padding
    let start_offset_x = (config.start_tick.0 as f32) * ppt;
    let origin_x = ruler_rect.min.x - scroll_x + config.left_padding - start_offset_x;

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

    let seek_key = ui.id().with("ruler_seeking");
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

    let pos_to_ticks = |pos_x: f32| -> Ticks {
        let raw_ticks = ((pos_x - origin_x) / ppt) as i64;
        Ticks(raw_ticks.max(0))
    };

    if seeking {
        if primary_down && let Some(pos) = hover_pos {
            ruler_res.seek_to = Some(pos_to_ticks(pos.x));
        }

        if primary_released {
            if let Some(pos) = hover_pos {
                ruler_res.seek_to = Some(pos_to_ticks(pos.x));
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

    let raw_interval = (40.0_f32 / ppb).ceil() as i32;
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

    // Calculate the visible range of beats
    let start_beat = (config.start_tick.0 as f64 / config.resolution as f64).floor() as i32;
    let left_beat = (((ruler_rect.min.x - origin_x) / ppb).floor() as i32).max(start_beat);
    let right_beat = ((ruler_rect.max.x - origin_x) / ppb).ceil() as i32;

    let tick_color = theme::ruler_color(dark_mode);
    let text_color = theme::ruler_label(dark_mode);

    // Define the number of minor lines between major beat lines based on the beats per label and ppb
    let subdivisions: i32 = if beats_per_label == 1 {
        if ppb >= 240.0 {
            8
        } else if ppb >= 120.0 {
            4
        } else if ppb >= 60.0 {
            2
        } else {
            1
        }
    } else {
        1
    };

    let unit_ppb = ppb / subdivisions as f32;
    let major_units = beats_per_label * subdivisions;
    let start_unit = (left_beat * subdivisions).max(start_beat * subdivisions);
    let end_unit = right_beat * subdivisions;

    // Render the ruler lines and labels
    for unit in start_unit..=end_unit {
        let x = origin_x + unit as f32 * unit_ppb;

        if unit % major_units == 0 {
            painter.vline(
                x,
                egui::Rangef::new(ruler_rect.min.y, ruler_rect.max.y),
                egui::Stroke::new(1.0, tick_color),
            );

            painter.text(
                egui::pos2(x + 3.0, ruler_rect.max.y - 2.0),
                egui::Align2::LEFT_BOTTOM,
                format!("{}", unit / subdivisions),
                egui::FontId::proportional(12.0),
                text_color,
            );
        } else if unit_ppb >= 20.0 {
            painter.vline(
                x,
                egui::Rangef::new(ruler_rect.max.y - MINOR_TICK_HEIGHT, ruler_rect.max.y),
                egui::Stroke::new(1.0, tick_color),
            );
        }
    }

    ruler_res
}
