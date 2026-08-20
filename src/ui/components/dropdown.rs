use crate::ui::{
    components::text_button::{text_button, text_button_enabled},
    theme,
};
use eframe::egui;
use std::hash::Hash;

pub(crate) fn dropdown_button<R>(
    ui: &mut egui::Ui,
    id: impl Hash,
    selected_text: impl Into<egui::WidgetText>,
    content: impl FnOnce(&mut egui::Ui) -> R,
) -> Option<egui::InnerResponse<R>> {
    let response = text_button(ui, id, selected_text);

    if response.clicked() {
        egui::Popup::toggle_id(&response.ctx, response.id);
    }

    egui::Popup::menu(&response)
        .style(theme::menu_style(ui))
        .show(content)
}

pub(crate) fn dropdown_button_enabled<R>(
    enabled: bool,
    ui: &mut egui::Ui,
    id: impl Hash,
    selected_text: impl Into<egui::WidgetText>,
    content: impl FnOnce(&mut egui::Ui) -> R,
) -> Option<egui::InnerResponse<R>> {
    let response = text_button_enabled(enabled, ui, id, selected_text);

    if response.clicked() {
        egui::Popup::toggle_id(&response.ctx, response.id);
    }

    egui::Popup::menu(&response)
        .style(theme::menu_style(ui))
        .show(content)
}
