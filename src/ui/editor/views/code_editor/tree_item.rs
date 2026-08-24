use crate::ui::theme;
use eframe::egui::{self, include_image};

const INDENT_SIZE: f32 = 12.0;
const ICON_SIZE: f32 = 18.0;
const HORIZONTAL_PADDING: f32 = 4.0;

pub(super) struct FileTreeItem<'a> {
    text: String,
    indent: i32,
    left_icon: Option<egui::ImageSource<'a>>,
    is_highlighted: bool,
}

impl<'a> FileTreeItem<'a> {
    pub(super) fn new(text: impl Into<String>, indent: i32) -> Self {
        Self {
            text: text.into(),
            indent,
            left_icon: None,
            is_highlighted: false,
        }
    }

    pub(super) fn with_left_icon(mut self, icon: egui::ImageSource<'a>) -> Self {
        self.left_icon = Some(icon);
        self
    }

    pub(super) fn collapsible(self, is_collapsed: bool) -> Self {
        if is_collapsed {
            self.with_left_icon(include_image!("../../../../../assets/icons/tri_right.svg"))
        } else {
            self.with_left_icon(include_image!("../../../../../assets/icons/tri_down.svg"))
        }
    }

    pub(super) fn highlighted(mut self, is_highlighted: bool) -> Self {
        self.is_highlighted = is_highlighted;
        self
    }
}

impl egui::Widget for FileTreeItem<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let size = ui.available_size();
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let bg_color = if self.is_highlighted {
                theme::selected_bg()
            } else if response.is_pointer_button_down_on() {
                theme::card_button_pressed(ui.visuals().dark_mode)
            } else if response.hovered() {
                theme::card_button_hovered(ui.visuals().dark_mode)
            } else {
                egui::Color32::TRANSPARENT
            };
            let text_color = if self.is_highlighted {
                theme::selected_fg()
            } else {
                theme::primary_fg(ui.visuals().dark_mode)
            };
            ui.painter().rect_filled(rect, 0, bg_color);

            let mut cursor_x = rect.min.x + HORIZONTAL_PADDING + self.indent as f32 * INDENT_SIZE;

            if let Some(icon) = self.left_icon {
                let icon_size = egui::Vec2::splat(ICON_SIZE);
                let icon_rect = egui::Rect::from_min_size(
                    egui::pos2(cursor_x, rect.center().y - icon_size.y * 0.5),
                    icon_size,
                );
                egui::Image::new(icon)
                    .tint(theme::primary_fg(ui.visuals().dark_mode))
                    .paint_at(ui, icon_rect);
            }
            cursor_x += ICON_SIZE + ui.spacing().icon_spacing;

            let text_rect = egui::Rect::from_min_max(egui::pos2(cursor_x, rect.min.y), rect.max);
            let galley = ui.painter().layout(
                self.text,
                egui::TextStyle::Body.resolve(ui.style()),
                text_color,
                text_rect.width().max(0.0),
            );
            let text_pos = egui::pos2(text_rect.min.x, rect.center().y - galley.size().y * 0.5);
            ui.painter().galley(text_pos, galley, text_color);
        }

        response
    }
}
