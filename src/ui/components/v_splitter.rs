use crate::ui::theme;
use eframe::egui;

pub(crate) const SPLITTER_WIDTH: f32 = 2.0;

pub(crate) struct VSplitter<'a> {
    width: &'a mut f32,
    min: f32,
    max: f32,
    height: Option<f32>,
}

impl<'a> VSplitter<'a> {
    pub(crate) fn new(width: &'a mut f32) -> Self {
        Self {
            width,
            min: 0.0,
            max: f32::INFINITY,
            height: None,
        }
    }

    pub(crate) fn with_min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    pub(crate) fn with_max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    pub(crate) fn with_height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub(crate) fn show(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let height = self
            .height
            .unwrap_or_else(|| ui.available_rect_before_wrap().height())
            .max(0.0);
        let (divider_rect, divider_resp) =
            ui.allocate_exact_size(egui::vec2(SPLITTER_WIDTH, height), egui::Sense::drag());

        // Handle divider drag and then draw the divider
        if divider_resp.dragged() {
            *self.width = (*self.width + divider_resp.drag_delta().x).clamp(self.min, self.max);
        }

        if divider_resp.hovered() {
            ui.set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
            ui.painter().rect_filled(
                divider_rect,
                0.0,
                theme::separator_hovered(ui.visuals().dark_mode),
            );
        } else {
            ui.painter()
                .rect_filled(divider_rect, 0.0, theme::separator(ui.visuals().dark_mode));
        }

        divider_resp
    }
}
