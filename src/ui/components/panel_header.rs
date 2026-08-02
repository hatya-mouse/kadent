use crate::ui::theme;
use eframe::egui;

pub(crate) fn panel_header<R>(
    ui: &mut egui::Ui,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    egui::Panel::top(ui.id().with("node_graph_header"))
        .frame(
            egui::Frame::new()
                .fill(theme::tertiary_bg(ui.visuals().dark_mode))
                .inner_margin(egui::Margin::symmetric(8, 4)),
        )
        .show_inside(ui, |ui| {
            ui.spacing_mut().button_padding = egui::vec2(0.0, 0.0);
            ui.horizontal_centered(|ui| add_contents(ui)).inner
        })
}
