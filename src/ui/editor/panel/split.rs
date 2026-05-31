use crate::ui_state::panel_layout::{PanelNode, SplitDir};
use crate::{theme, ui::EditorUi};
use eframe::egui::{self, CursorIcon, Rect};

const DIVIDER_SIZE: f32 = 4.0;
const MIN_COLLAPSE_SIZE: f32 = 60.0;

/// Returns Some(keep_first) when a panel should be collapsed on drag release.
pub(super) fn render_split(
    ui: &mut egui::Ui,
    dir: SplitDir,
    ratio: &mut f32,
    first: &mut PanelNode,
    second: &mut PanelNode,
    rect: Rect,
    editor: &mut EditorUi,
) -> Option<bool> {
    let (first_rect, div_rect, second_rect) = split_rects(rect, dir, *ratio);
    if panel_size(dir, first_rect) >= 1.0 {
        super::render_node(ui, first, first_rect, editor);
    }
    if panel_size(dir, second_rect) >= 1.0 {
        super::render_node(ui, second, second_rect, editor);
    }
    render_divider(ui, div_rect, dir, ratio, rect)
}

fn split_rects(rect: Rect, dir: SplitDir, ratio: f32) -> (Rect, Rect, Rect) {
    let half = DIVIDER_SIZE / 2.0;
    match dir {
        SplitDir::Vertical => {
            let x = rect.min.x + rect.width() * ratio;
            let first = Rect::from_min_max(rect.min, egui::pos2(x - half, rect.max.y));
            let div = Rect::from_min_max(
                egui::pos2(x - half, rect.min.y),
                egui::pos2(x + half, rect.max.y),
            );
            let second = Rect::from_min_max(egui::pos2(x + half, rect.min.y), rect.max);
            (first, div, second)
        }
        SplitDir::Horizontal => {
            let y = rect.min.y + rect.height() * ratio;
            let first = Rect::from_min_max(rect.min, egui::pos2(rect.max.x, y - half));
            let div = Rect::from_min_max(
                egui::pos2(rect.min.x, y - half),
                egui::pos2(rect.max.x, y + half),
            );
            let second = Rect::from_min_max(egui::pos2(rect.min.x, y + half), rect.max);
            (first, div, second)
        }
    }
}

fn panel_size(dir: SplitDir, rect: Rect) -> f32 {
    match dir {
        SplitDir::Vertical => rect.width(),
        SplitDir::Horizontal => rect.height(),
    }
}

fn render_divider(
    ui: &mut egui::Ui,
    div_rect: Rect,
    dir: SplitDir,
    ratio: &mut f32,
    total: Rect,
) -> Option<bool> {
    let resp = ui.allocate_rect(div_rect, egui::Sense::drag());

    let color = if resp.hovered() || resp.dragged() {
        theme::separator_hovered()
    } else {
        theme::separator()
    };
    ui.painter().rect_filled(div_rect, 0.0, color);

    if resp.hovered() || resp.dragged() {
        ui.ctx().set_cursor_icon(match dir {
            SplitDir::Vertical => CursorIcon::ResizeHorizontal,
            SplitDir::Horizontal => CursorIcon::ResizeVertical,
        });
    }

    if resp.dragged() {
        let delta = match dir {
            SplitDir::Vertical => resp.drag_delta().x / total.width(),
            SplitDir::Horizontal => resp.drag_delta().y / total.height(),
        };
        // Allow ratio past 0.1/0.9 so panels can enter the collapse zone
        *ratio = (*ratio + delta).clamp(0.0, 1.0);

        draw_close_overlays(ui, dir, *ratio, total);
    }

    if resp.drag_stopped() {
        let (f_rect, _, s_rect) = split_rects(total, dir, *ratio);
        if panel_size(dir, f_rect) < MIN_COLLAPSE_SIZE {
            return Some(false); // If the first is too small -> keep second
        }
        if panel_size(dir, s_rect) < MIN_COLLAPSE_SIZE {
            return Some(true); // If the second is too small -> keep first
        }
    }

    None
}

fn draw_close_overlays(ui: &mut egui::Ui, dir: SplitDir, ratio: f32, total: Rect) {
    let (f_rect, _, s_rect) = split_rects(total, dir, ratio);
    let close_color = theme::panel_collapse_overlay();
    if panel_size(dir, f_rect) < MIN_COLLAPSE_SIZE {
        ui.painter().rect_filled(f_rect, 0.0, close_color);
    }
    if panel_size(dir, s_rect) < MIN_COLLAPSE_SIZE {
        ui.painter().rect_filled(s_rect, 0.0, close_color);
    }
}
