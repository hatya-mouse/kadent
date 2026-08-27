use crate::ui::theme;
use eframe::egui;

pub(crate) const SPLITTER_HEIGHT: f32 = 2.0;

pub(crate) struct HSplitter<'a> {
    height: &'a mut f32,
    min: f32,
    max: f32,
    width: Option<f32>,
}

impl<'a> HSplitter<'a> {
    pub(crate) fn new(height: &'a mut f32) -> Self {
        Self {
            height,
            min: 0.0,
            max: f32::INFINITY,
            width: None,
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

    pub(crate) fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub(crate) fn show(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let width = self
            .width
            .unwrap_or_else(|| ui.available_rect_before_wrap().width())
            .max(0.0);

        let (divider_rect, divider_resp) =
            ui.allocate_exact_size(egui::vec2(width, SPLITTER_HEIGHT), egui::Sense::drag());

        if divider_resp.dragged() {
            *self.height = (*self.height - divider_resp.drag_delta().y).clamp(self.min, self.max);
        }

        if divider_resp.hovered() {
            ui.set_cursor_icon(egui::CursorIcon::ResizeVertical);
        }

        ui.painter().rect_filled(
            divider_rect,
            0.0,
            if divider_resp.hovered() {
                theme::separator_hovered(ui.visuals().dark_mode)
            } else {
                theme::separator(ui.visuals().dark_mode)
            },
        );

        divider_resp
    }
}
