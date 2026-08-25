use crate::fonts::RichTextExt;
use eframe::egui;

// --- FONT SIZE ---

pub(crate) fn normal_font_size() -> f32 {
    14.0
}

pub(crate) fn large_font_size() -> f32 {
    16.0
}

pub(crate) fn toolbar_beats_font_size() -> f32 {
    18.0
}

// --- NOT AVAILABLE LABEL ---

pub(crate) fn not_available_label(text: &str) -> egui::RichText {
    egui::RichText::new(text)
        .size(normal_font_size())
        .weak()
        .bold()
}
