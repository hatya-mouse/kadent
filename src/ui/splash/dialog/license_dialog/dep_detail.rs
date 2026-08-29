use crate::{
    fonts::RichTextExt,
    ui::{
        components::not_available_text::not_available_text,
        splash::dialog::{
            LicenseDialogState,
            license_dialog::{LICENSE_MARGIN, SelectedItem},
        },
    },
};
use eframe::egui;

impl LicenseDialogState {
    pub(super) fn dep_detail(&mut self, ui: &mut egui::Ui) {
        let Ok(document) = &self.document else {
            return;
        };

        let Some(dep) = self.selected_dep.as_ref().and_then(|item| match item {
            SelectedItem::Font(index) => document.fonts.get(*index),
            SelectedItem::Crate(index) => document.crates.get(*index),
        }) else {
            not_available_text(ui, "Select an item to view its details.");
            return;
        };

        egui::ScrollArea::vertical()
            .id_salt("dep_detail")
            .content_margin(LICENSE_MARGIN)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.take_available_width();

                    ui.label(egui::RichText::new(&dep.name).heading().bold());
                    dep_field_option(ui, "Version", dep.version.as_ref());
                    dep_field_option(
                        ui,
                        "Author(s)",
                        dep.authors
                            .as_ref()
                            .map(|authors| authors.join(", "))
                            .as_ref(),
                    );
                    dep_field_option(ui, "Description", dep.description.as_ref());

                    let license = document.licenses.get(dep.license_index);
                    let should_show_separator = (dep.version.is_some()
                        || dep
                            .authors
                            .as_ref()
                            .is_some_and(|authors| !authors.is_empty())
                        || dep.description.is_some())
                        && (license.is_some() || dep.notice.is_some());
                    if should_show_separator {
                        ui.separator();
                    }

                    // Show the license
                    if let Some(license) = license {
                        dep_field(ui, "License", &format!("{} / {}", license.id, license.name));
                        ui.add_space(4.0);
                        ui.add(
                            egui::Label::new(egui::RichText::new(&license.text).monospace())
                                .wrap()
                                .selectable(true),
                        );
                    }

                    // Show notice if it has any
                    if let Some(notice) = dep.notice.as_ref() {
                        ui.label(egui::RichText::new("Notice").bold());
                        ui.add_space(4.0);
                        ui.add(
                            egui::Label::new(egui::RichText::new(notice).monospace())
                                .wrap()
                                .selectable(true),
                        );
                    }
                });
            });
    }
}

fn dep_field_option(ui: &mut egui::Ui, label: &str, value: Option<&String>) {
    if let Some(value) = value
        && !value.is_empty()
    {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(label).bold());
            ui.add_space(8.0);
            ui.add(
                egui::Label::new(egui::RichText::new(value).monospace())
                    .wrap()
                    .selectable(true),
            );
        });
    }
}

fn dep_field(ui: &mut egui::Ui, label: &str, value: &str) {
    if !value.is_empty() {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(label).bold());
            ui.add_space(8.0);
            ui.add(
                egui::Label::new(egui::RichText::new(value).monospace())
                    .wrap()
                    .selectable(true),
            );
        });
    }
}
