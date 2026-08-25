use crate::{fonts::RichTextExt, ui::theme};
use eframe::egui::{self, CornerRadius, ModalResponse, Style};

const DIALOG_SPACING: egui::Vec2 = egui::vec2(8.0, 6.0);

pub(crate) fn dialog<R>(
    ui: &egui::Ui,
    title: impl Into<String>,
    content: impl FnOnce(&mut egui::Ui) -> R,
) -> ModalResponse<R> {
    egui::Modal::new(ui.id().with("dialog"))
        .frame(
            egui::Frame::popup(ui.style())
                .shadow(egui::Shadow::NONE)
                .inner_margin(0),
        )
        .show(ui, |ui| {
            // Show the dialog title header
            let corner_radius = ui.style().visuals.window_corner_radius;
            egui::Frame::new()
                .fill(theme::tertiary_bg(ui.visuals().dark_mode))
                .inner_margin(0)
                .corner_radius(CornerRadius {
                    nw: corner_radius.nw,
                    ne: corner_radius.ne,
                    sw: 0,
                    se: 0,
                })
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new(title)
                                .size(theme::large_font_size())
                                .color(theme::primary_fg(ui.visuals().dark_mode))
                                .bold(),
                        );
                        ui.add_space(1.0);
                        ui.add(egui::Separator::default().spacing(0.0));
                    });
                });

            // Show the content
            *ui.style_mut() = dialog_style(ui);
            egui::Frame::new().inner_margin(6).show(ui, content).inner
        })
}

pub(crate) fn dialog_bold_label(ui: &mut egui::Ui, text: impl Into<String>) {
    ui.label(
        egui::RichText::new(text)
            .size(theme::normal_font_size())
            .color(theme::primary_fg(ui.visuals().dark_mode))
            .bold(),
    );
}

fn dialog_style(ui: &egui::Ui) -> Style {
    let mut dialog_style = ui.style().as_ref().clone();

    // --- MENU STYLES ---
    let dark_mode = dialog_style.visuals.dark_mode;
    dialog_style.spacing.item_spacing = DIALOG_SPACING;
    dialog_style.spacing.interact_size = egui::vec2(0.0, 0.0);

    // --- MENU ITEM STYLES ---
    let primary_stroke = egui::Stroke::new(1.0, theme::primary_fg(dark_mode));
    let selected_stroke = egui::Stroke::new(1.0, theme::selected_fg());

    let selection = &mut dialog_style.visuals.selection;
    selection.bg_fill = theme::selected_bg();
    selection.stroke = selected_stroke;

    let hovered = &mut dialog_style.visuals.widgets.hovered;
    hovered.weak_bg_fill = theme::button_bg(dark_mode);
    hovered.bg_stroke = egui::Stroke::NONE;
    hovered.fg_stroke = primary_stroke;

    let active = &mut dialog_style.visuals.widgets.active;
    active.weak_bg_fill = theme::selected_bg();
    active.bg_stroke = egui::Stroke::NONE;
    active.fg_stroke = selected_stroke;

    let inactive = &mut dialog_style.visuals.widgets.inactive;
    inactive.bg_stroke = egui::Stroke::NONE;
    inactive.fg_stroke = primary_stroke;

    let open = &mut dialog_style.visuals.widgets.open;
    open.weak_bg_fill = theme::button_bg(dark_mode);
    open.bg_stroke = egui::Stroke::NONE;
    open.fg_stroke = primary_stroke;

    let noninteractive = &mut dialog_style.visuals.widgets.noninteractive;
    noninteractive.bg_fill = theme::button_bg(dark_mode);
    noninteractive.bg_stroke = egui::Stroke::NONE;
    noninteractive.fg_stroke = primary_stroke;

    dialog_style
}
