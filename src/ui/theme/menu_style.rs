use crate::ui::theme;
use eframe::egui::{self, Style};

const ITEM_CORNER_RADIUS: egui::CornerRadius = egui::CornerRadius::same(4);
const MENU_CORNER_RADIUS: egui::CornerRadius = egui::CornerRadius::same(7);
const MENU_SPACING: f32 = 3.0;
const MENU_MARGIN: i8 = 4;
const MENU_ITEM_PADDING: egui::Vec2 = egui::vec2(5.0, 3.0);

pub(crate) fn menu_style(ui: &egui::Ui) -> Style {
    let mut menu_style = ui.style().as_ref().clone();

    // --- MENU STYLES ---
    let dark_mode = menu_style.visuals.dark_mode;
    menu_style.visuals.menu_corner_radius = MENU_CORNER_RADIUS;
    menu_style.spacing.button_padding = MENU_ITEM_PADDING;
    menu_style.spacing.menu_margin = egui::Margin::same(MENU_MARGIN);
    menu_style.spacing.menu_spacing = MENU_SPACING;
    menu_style.spacing.item_spacing = egui::vec2(MENU_SPACING, MENU_SPACING);
    menu_style.spacing.interact_size = egui::vec2(0.0, 0.0);

    // --- MENU ITEM STYLES ---
    let primary_stroke = egui::Stroke::new(1.0, theme::primary_fg(dark_mode));
    let selected_stroke = egui::Stroke::new(1.0, theme::selected_fg());

    let selection = &mut menu_style.visuals.selection;
    selection.bg_fill = theme::selected_bg();
    selection.stroke = selected_stroke;

    let hovered = &mut menu_style.visuals.widgets.hovered;
    hovered.weak_bg_fill = theme::button_bg(dark_mode);
    hovered.bg_stroke = egui::Stroke::NONE;
    hovered.fg_stroke = primary_stroke;
    hovered.corner_radius = ITEM_CORNER_RADIUS;

    let active = &mut menu_style.visuals.widgets.active;
    active.weak_bg_fill = theme::selected_bg();
    active.bg_stroke = egui::Stroke::NONE;
    active.fg_stroke = selected_stroke;
    active.corner_radius = ITEM_CORNER_RADIUS;

    let inactive = &mut menu_style.visuals.widgets.inactive;
    inactive.bg_stroke = egui::Stroke::NONE;
    inactive.fg_stroke = primary_stroke;
    inactive.corner_radius = ITEM_CORNER_RADIUS;

    let open = &mut menu_style.visuals.widgets.open;
    open.weak_bg_fill = theme::button_bg(dark_mode);
    open.bg_stroke = egui::Stroke::NONE;
    open.fg_stroke = primary_stroke;
    open.corner_radius = ITEM_CORNER_RADIUS;

    let noninteractive = &mut menu_style.visuals.widgets.noninteractive;
    noninteractive.bg_fill = theme::button_bg(dark_mode);
    noninteractive.bg_stroke = egui::Stroke::NONE;
    noninteractive.fg_stroke = primary_stroke;
    noninteractive.corner_radius = ITEM_CORNER_RADIUS;

    menu_style
}
