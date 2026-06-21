use crate::ui::theme;
use eframe::egui::{self, Style};

pub(crate) fn menu_style(ui: &egui::Ui) -> Style {
    let mut menu_style = ui.style().as_ref().clone();

    let dark_mode = menu_style.visuals.dark_mode;
    menu_style.visuals.menu_corner_radius = egui::CornerRadius::same(6);
    menu_style.spacing.button_padding = egui::vec2(3.0, 3.0);
    menu_style.spacing.menu_margin = egui::Margin::same(3);
    menu_style.spacing.interact_size = egui::vec2(0.0, 0.0);

    let primary_stroke = egui::Stroke::new(1.0, theme::primary_fg(dark_mode));
    let selected_stroke = egui::Stroke::new(1.0, theme::selected_fg());

    let selection = &mut menu_style.visuals.selection;
    selection.bg_fill = theme::selected_bg();
    selection.stroke = selected_stroke;

    let hovered = &mut menu_style.visuals.widgets.hovered;
    hovered.weak_bg_fill = theme::button_bg(dark_mode);
    hovered.bg_stroke = egui::Stroke::NONE;
    hovered.fg_stroke = primary_stroke;

    let active = &mut menu_style.visuals.widgets.active;
    active.weak_bg_fill = theme::selected_bg();
    active.bg_stroke = egui::Stroke::NONE;
    active.fg_stroke = selected_stroke;

    let inactive = &mut menu_style.visuals.widgets.inactive;
    inactive.bg_stroke = egui::Stroke::NONE;
    inactive.fg_stroke = primary_stroke;

    let open = &mut menu_style.visuals.widgets.open;
    open.weak_bg_fill = theme::button_bg(dark_mode);
    open.bg_stroke = egui::Stroke::NONE;
    open.fg_stroke = primary_stroke;

    let noninteractive = &mut menu_style.visuals.widgets.noninteractive;
    noninteractive.bg_fill = theme::button_bg(dark_mode);
    noninteractive.bg_stroke = egui::Stroke::NONE;
    noninteractive.fg_stroke = primary_stroke;

    menu_style
}
