use crate::ui::theme;
use eframe::egui;

const MINIMUM_HANDLE_WIDTH: f32 = 12.0;

/// Draws a scroll bar of the timeline and returns the new scroll position if the scroll bar handle is dragged.
pub(super) fn scroll_bar(
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
