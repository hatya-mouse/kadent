use crate::ui::{components::card_button::card_button, theme};
use eframe::egui;
use std::hash::Hash;

pub(crate) fn dropdown_button<R>(
    ui: &mut egui::Ui,
    id: impl Hash,
    selected_text: impl Into<String>,
    content: impl FnOnce(&mut egui::Ui) -> R,
) -> Option<egui::InnerResponse<R>> {
    let response = card_button(ui, ui.id().with(id), None, |ui| {
        ui.add(egui::Label::new(
            egui::RichText::new(selected_text).strong(),
        ));
        ui.add(egui::Image::new(egui::include_image!(
            "../../../assets/icons/tri_down.svg"
        )));
    });

    if response.clicked() {
        egui::Popup::toggle_id(&response.ctx, response.id);
    }

    egui::Popup::menu(&response)
        .style(theme::menu_style(ui))
        .show(content)
}
