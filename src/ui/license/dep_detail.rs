use crate::{
    fonts::RichTextExt,
    ui::{LicenseUi, components::not_available_text::not_available_text, license::SelectedItem},
};
use eframe::egui;

impl LicenseUi {
    pub(super) fn dep_detail(&mut self, ui: &mut egui::Ui) {
        let Some(dep) = &self.selected_dep.and_then(|item| match item {
            SelectedItem::Font(index) => self.fonts.get(index),
            SelectedItem::Crate(index) => self.crates.get(index),
        }) else {
            not_available_text(ui, "Select an item to view its details.");
            return;
        };

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical(|ui| {
                ui.label(egui::RichText::new(&dep.name).heading().bold());
                dep_field(ui, "Version", &dep.version);
                dep_field(ui, "Author(s)", &dep.authors.join(", "));
                dep_field(ui, "License", &dep.license.to_string());
                ui.separator();
                dep_field(ui, "Copyright", &dep.copyright);

                ui.label(egui::RichText::new("License Text").bold());
                ui.add_space(4.0);
                ui.label(egui::RichText::new(license_text(&dep.license)).monospace());

                // Show notice if it has any
                if let Some(notice) = dep.notice.as_ref() {
                    ui.label(egui::RichText::new("Notice").bold());
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new(notice).monospace());
                }
            });
        });
    }
}

fn dep_field(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).bold());
        ui.add_space(8.0);
        ui.label(egui::RichText::new(value).monospace());
    });
}
