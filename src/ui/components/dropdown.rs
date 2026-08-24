use crate::ui::{components::card_button::card_button, theme};
use eframe::egui;
use std::hash::Hash;

const ICON_SIZE: f32 = 18.0;

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
        ui.add(
            egui::Image::new(egui::include_image!("../../../assets/icons/tri_down.svg"))
                .fit_to_exact_size(egui::Vec2::splat(ICON_SIZE))
                .tint(theme::primary_fg(ui.visuals().dark_mode)),
        );
    });

    if response.clicked() {
        egui::Popup::toggle_id(&response.ctx, response.id);
    }

    egui::Popup::menu(&response)
        .style(theme::menu_style(ui))
        .show(content)
}
