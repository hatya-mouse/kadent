use crate::ui::components::card_button::{card_button, card_button_enabled};
use eframe::egui;
use std::hash::Hash;

pub(crate) fn text_button_enabled(
    enabled: bool,
    ui: &mut egui::Ui,
    id: impl Hash,
    text: impl Into<egui::WidgetText>,
) -> egui::Response {
    card_button_enabled(enabled, ui, ui.id().with(id), None, |ui| {
        ui.add(egui::Label::new(text))
    })
}

pub(crate) fn text_button(
    ui: &mut egui::Ui,
    id: impl Hash,
    text: impl Into<egui::WidgetText>,
) -> egui::Response {
    card_button(ui, ui.id().with(id), None, |ui| {
        ui.add(egui::Label::new(text))
    })
}
