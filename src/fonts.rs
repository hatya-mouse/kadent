use crate::app::KadentApp;
use eframe::egui::{self, FontData, FontDefinitions, FontFamily, RichText};
use std::sync::Arc;

pub(crate) static FONT_FAMILY_BOLD: &str = "bold";
pub(crate) static FONT_FAMILY_BOLD_MONO: &str = "mono_bold";

pub(crate) fn font_family_bold() -> FontFamily {
    FontFamily::Name(Arc::from(FONT_FAMILY_BOLD))
}

pub(crate) fn font_family_bold_mono() -> FontFamily {
    FontFamily::Name(Arc::from(FONT_FAMILY_BOLD_MONO))
}

pub(crate) trait RichTextExt {
    fn bold(self) -> Self;

    fn bold_mono(self) -> Self;
}

impl RichTextExt for RichText {
    fn bold(self) -> Self {
        self.family(font_family_bold())
    }

    fn bold_mono(self) -> Self {
        self.family(font_family_bold_mono())
    }
}

impl KadentApp {
    pub(crate) fn setup_fonts(ctx: &egui::Context) {
        // Set up the fonts
        let mut fonts = FontDefinitions::default();

        let inter_regular =
            FontData::from_static(include_bytes!("../assets/fonts/Inter-Regular.ttf"));
        fonts
            .font_data
            .insert("Inter-Regular".to_owned(), Arc::new(inter_regular));
        let inter_bold = FontData::from_static(include_bytes!("../assets/fonts/Inter-Bold.ttf"));
        fonts
            .font_data
            .insert("Inter-Bold".to_owned(), Arc::new(inter_bold));

        let noto_sans_jp_regular =
            FontData::from_static(include_bytes!("../assets/fonts/NotoSansJP-Regular.ttf"));
        fonts.font_data.insert(
            "NotoSansJP-Regular".to_owned(),
            Arc::new(noto_sans_jp_regular),
        );
        let noto_sans_jp_bold =
            FontData::from_static(include_bytes!("../assets/fonts/NotoSansJP-Bold.ttf"));
        fonts
            .font_data
            .insert("NotoSansJP-Bold".to_owned(), Arc::new(noto_sans_jp_bold));

        let inter_regular =
            FontData::from_static(include_bytes!("../assets/fonts/RobotoMono-Regular.ttf"));
        fonts
            .font_data
            .insert("Roboto-Regular".to_owned(), Arc::new(inter_regular));
        let inter_bold =
            FontData::from_static(include_bytes!("../assets/fonts/RobotoMono-SemiBold.ttf"));
        fonts
            .font_data
            .insert("Roboto-SemiBold".to_owned(), Arc::new(inter_bold));

        // Set the font data
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "Inter-Regular".to_string());
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(1, "NotoSansJP-Regular".to_string());
        fonts
            .families
            .entry(FontFamily::Monospace)
            .or_default()
            .insert(0, "Roboto-Regular".to_string());
        fonts
            .families
            .entry(FontFamily::Monospace)
            .or_default()
            .insert(1, "NotoSansJP-Regular".to_string());

        fonts
            .families
            .entry(font_family_bold())
            .or_default()
            .insert(0, "Inter-Bold".to_string());
        fonts
            .families
            .entry(font_family_bold())
            .or_default()
            .insert(1, "NotoSansJP-Bold".to_string());
        fonts
            .families
            .entry(font_family_bold_mono())
            .or_default()
            .insert(0, "Roboto-SemiBold".to_string());
        fonts
            .families
            .entry(font_family_bold_mono())
            .or_default()
            .insert(1, "NotoSansJP-Bold".to_string());

        // Set the fonts to the specified fonts
        ctx.set_fonts(fonts);
    }
}
