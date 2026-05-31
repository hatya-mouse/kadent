use super::SplitAction;
use crate::{
    theme,
    ui::EditorUi,
    ui_state::panel_layout::{PanelView, SplitDir},
};
use eframe::egui::{self, CursorIcon, Rect, UiBuilder};

const EDGE_SIZE: f32 = 10.0;
const MIN_NEW_PANEL: f32 = 80.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Edge {
    Left,
    Right,
    Top,
    Bottom,
}

pub(super) fn render_leaf(
    ui: &mut egui::Ui,
    view: &mut PanelView,
    rect: Rect,
    editor: &mut EditorUi,
) -> Option<SplitAction> {
    let salt = (rect.min.x as i32, rect.min.y as i32);

    // Use a scoped child UI with a unique ID derived from the panel position
    // This ensures scroll state and widget IDs are independent per panel
    let result = ui.scope_builder(UiBuilder::new().id_salt(salt).max_rect(rect), |ui| {
        ui.set_clip_rect(rect);

        // Remove the spacing between header and content
        ui.spacing_mut().item_spacing.y = 0.0;

        render_header(ui, view);

        // Clip the content to the area below the header
        // Without this, painter-based content (node graph, piano roll) would draw
        // over the header because the outer clip rect covers the full panel
        let content_rect = ui.available_rect_before_wrap();
        ui.scope_builder(UiBuilder::new().max_rect(content_rect), |ui| {
            ui.set_clip_rect(content_rect);
            render_view_content(ui, view, editor);
        });

        check_edge_drag(ui, rect)
    });

    if ui.rect_contains_pointer(rect) {
        ui.painter().rect_stroke(
            rect,
            0.0,
            egui::Stroke::new(2.0, theme::border(ui.visuals().dark_mode)),
            egui::StrokeKind::Inside,
        );
    }

    result.inner
}

fn render_header(ui: &mut egui::Ui, view: &mut PanelView) {
    let response = egui::Frame::new()
        .fill(theme::tertiary_bg(ui.visuals().dark_mode))
        .inner_margin(egui::Margin::symmetric(8, 4))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());

            egui::ComboBox::from_id_salt("panel_selection_combo_box")
                .selected_text(view.to_string())
                .show_ui(ui, |ui| {
                    PanelView::all().iter().for_each(|enum_view| {
                        ui.selectable_value(view, enum_view.clone(), enum_view.to_string());
                    });
                })
        });

    let stroke = ui.visuals().widgets.noninteractive.bg_stroke;
    let rect = response.response.rect;

    // Adjust the y coordinate to align with the bottom edge of the header frame
    ui.painter().line_segment(
        [
            rect.left_bottom() - egui::vec2(0.0, 0.5),
            rect.right_bottom() - egui::vec2(0.0, 0.5),
        ],
        stroke,
    );
}

fn render_view_content(ui: &mut egui::Ui, view: &PanelView, editor: &mut EditorUi) {
    match view {
        PanelView::Timeline => editor.timeline(ui),
        PanelView::PianoRoll => editor.piano_roll(ui),
        PanelView::NodeGraph => editor.node_graph(ui),
        PanelView::Inspector => editor.inspector(ui),
        PanelView::ErrorList => editor.error_list(ui),
        PanelView::CodeEditor => editor.code_editor(ui),
    }
}

fn check_edge_drag(ui: &mut egui::Ui, rect: Rect) -> Option<SplitAction> {
    for edge in [Edge::Left, Edge::Right, Edge::Top, Edge::Bottom] {
        if let Some(action) = check_single_edge(ui, rect, edge) {
            return Some(action);
        }
    }
    None
}

fn check_single_edge(ui: &mut egui::Ui, rect: Rect, edge: Edge) -> Option<SplitAction> {
    let strip = edge_strip(rect, edge);
    let id = ui.id().with("panel_edge").with(edge);

    let resp = ui.interact(strip, id, egui::Sense::drag());

    if resp.hovered() || resp.dragged() {
        ui.ctx().set_cursor_icon(edge_cursor(edge));
    }

    let accum_key = id.with("accum");
    let prev: f32 = ui.data(|d| d.get_temp(accum_key).unwrap_or(0.0));

    if resp.dragged() {
        let new_accum = prev + inward_delta(edge, resp.drag_delta());
        ui.data_mut(|d| d.insert_temp(accum_key, new_accum));

        // Draw edge highlight
        ui.painter()
            .rect_filled(strip, 0.0, theme::panel_drag_highlight());

        // Draw preview of the new panel
        let preview_size = new_accum.clamp(0.0, panel_extent(edge, rect) - MIN_NEW_PANEL);
        if preview_size > 0.0 {
            let preview = new_panel_rect(rect, edge, preview_size);
            ui.painter()
                .rect_filled(preview, 0.0, theme::panel_hover_highlight());
        }
    } else if resp.hovered() {
        ui.painter()
            .rect_filled(strip, 0.0, theme::panel_hover_highlight());
    }

    if resp.drag_stopped() {
        ui.data_mut(|d| d.remove::<f32>(accum_key));
        if prev >= MIN_NEW_PANEL {
            let extent = panel_extent(edge, rect);
            let clamped = prev.min(extent - MIN_NEW_PANEL);
            return Some(build_split_action(edge, clamped, extent));
        }
    }

    None
}

fn edge_strip(rect: Rect, edge: Edge) -> Rect {
    match edge {
        Edge::Left => Rect::from_min_size(rect.min, egui::vec2(EDGE_SIZE, rect.height())),
        Edge::Right => Rect::from_min_size(
            egui::pos2(rect.max.x - EDGE_SIZE, rect.min.y),
            egui::vec2(EDGE_SIZE, rect.height()),
        ),
        Edge::Top => Rect::from_min_size(rect.min, egui::vec2(rect.width(), EDGE_SIZE)),
        Edge::Bottom => Rect::from_min_size(
            egui::pos2(rect.min.x, rect.max.y - EDGE_SIZE),
            egui::vec2(rect.width(), EDGE_SIZE),
        ),
    }
}

fn edge_cursor(edge: Edge) -> CursorIcon {
    match edge {
        Edge::Left | Edge::Right => CursorIcon::ResizeHorizontal,
        Edge::Top | Edge::Bottom => CursorIcon::ResizeVertical,
    }
}

fn inward_delta(edge: Edge, delta: egui::Vec2) -> f32 {
    match edge {
        Edge::Left => delta.x,
        Edge::Right => -delta.x,
        Edge::Top => delta.y,
        Edge::Bottom => -delta.y,
    }
}

fn panel_extent(edge: Edge, rect: Rect) -> f32 {
    match edge {
        Edge::Left | Edge::Right => rect.width(),
        Edge::Top | Edge::Bottom => rect.height(),
    }
}

fn new_panel_rect(rect: Rect, edge: Edge, size: f32) -> Rect {
    match edge {
        Edge::Left => Rect::from_min_size(rect.min, egui::vec2(size, rect.height())),
        Edge::Right => Rect::from_min_max(egui::pos2(rect.max.x - size, rect.min.y), rect.max),
        Edge::Top => Rect::from_min_size(rect.min, egui::vec2(rect.width(), size)),
        Edge::Bottom => Rect::from_min_max(egui::pos2(rect.min.x, rect.max.y - size), rect.max),
    }
}

fn build_split_action(edge: Edge, new_size: f32, total: f32) -> SplitAction {
    let ratio = match edge {
        Edge::Left => new_size / total,
        Edge::Right => 1.0 - new_size / total,
        Edge::Top => new_size / total,
        Edge::Bottom => 1.0 - new_size / total,
    }
    .clamp(0.1, 0.9);

    SplitAction {
        dir: match edge {
            Edge::Left | Edge::Right => SplitDir::Vertical,
            Edge::Top | Edge::Bottom => SplitDir::Horizontal,
        },
        ratio,
        new_panel_first: matches!(edge, Edge::Left | Edge::Top),
    }
}
