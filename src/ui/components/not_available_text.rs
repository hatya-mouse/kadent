use crate::{fonts::RichTextExt, ui::theme};
use eframe::egui;

pub(crate) fn not_available_text(ui: &mut egui::Ui, text: impl Into<String>) {
    ui.with_layout(
        egui::Layout::centered_and_justified(egui::Direction::TopDown),
        |ui| {
            ui.label(
                egui::RichText::new(text.into().as_str())
                    .size(theme::normal_font_size())
                    .weak()
                    .bold(),
            )
        },
    );
}
