use crate::{fonts::RichTextExt, ui::theme};
use eframe::egui::{self, CornerRadius, ModalResponse};

pub(crate) fn dialog<R>(
    ui: &egui::Ui,
    title: impl Into<String>,
    content: impl FnOnce(&mut egui::Ui) -> R,
) -> ModalResponse<R> {
    egui::Modal::new(egui::Id::new("add_track"))
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
                    egui::Frame::new()
                        .inner_margin(egui::Margin {
                            left: 8,
                            right: 8,
                            top: 4,
                            bottom: 2,
                        })
                        .show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(title)
                                    .size(theme::large_font_size())
                                    .color(theme::primary_fg(ui.visuals().dark_mode))
                                    .bold(),
                            );
                        });

                    ui.add(egui::Separator::default().spacing(0.0));
                });

            // Show the content
            content(ui)
        })
}
