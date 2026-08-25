use crate::ui::components::card_button::{
    card_button, card_button_enabled, card_button_highlighted,
};
use eframe::egui::{self, AsIdSalt};

pub(crate) fn text_button_enabled(
    enabled: bool,
    highlighted: bool,
    ui: &mut egui::Ui,
    id: impl AsIdSalt,
    text: impl Into<egui::WidgetText>,
) -> egui::Response {
    card_button_enabled(enabled, highlighted, ui, ui.id().with(id), None, |ui| {
        ui.add(egui::Label::new(text))
    })
}

pub(crate) fn text_button(
    ui: &mut egui::Ui,
    id: impl AsIdSalt,
    text: impl Into<egui::WidgetText>,
) -> egui::Response {
    card_button(ui, ui.id().with(id), None, |ui| {
        ui.add(egui::Label::new(text))
    })
}

pub(crate) fn text_button_highlighted(
    ui: &mut egui::Ui,
    id: impl AsIdSalt,
    text: impl Into<egui::WidgetText>,
) -> egui::Response {
    card_button_highlighted(ui, ui.id().with(id), None, |ui| {
        ui.add(egui::Label::new(text))
    })
}
