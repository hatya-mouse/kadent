use crate::{
    fonts::RichTextExt,
    ui::{
        components::not_available_text::not_available_text,
        splash::dialog::{
            LicenseDialogState,
            license_dialog::{SelectedItem, crate_licenses::DependencyItem},
        },
        theme,
    },
};
use eframe::egui;

impl LicenseDialogState {
    pub(super) fn dep_list(&mut self, ui: &mut egui::Ui) {
        let Ok(document) = &self.document else {
            not_available_text(ui, "Failed to load licenses.");
            return;
        };

        *ui.style_mut() = theme::menu_style(ui);

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical(|ui| {
                // --- FONTS ---
                ui.label(egui::RichText::new("Fonts").bold().size(18.0));
                for (index, font_item) in document.fonts.iter().enumerate() {
                    dep_list_item(ui, &mut self.selected_dep, font_item, index, true);
                }

                // --- CRATES ---
                ui.label(egui::RichText::new("Crates").bold().size(18.0));
                for (index, crate_item) in document.crates.iter().enumerate() {
                    dep_list_item(ui, &mut self.selected_dep, crate_item, index, false);
                }
            });
        });
    }
}

fn dep_list_item(
    ui: &mut egui::Ui,
    selected_dep: &mut Option<SelectedItem>,
    item: &DependencyItem,
    index: usize,
    is_font: bool,
) {
    let is_selected = match selected_dep {
        Some(SelectedItem::Font(i)) if is_font => *i == index,
        Some(SelectedItem::Crate(i)) if !is_font => *i == index,
        _ => false,
    };

    let response = ui.selectable_label(is_selected, &item.name);
    if response.clicked() {
        *selected_dep = if is_font {
            Some(SelectedItem::Font(index))
        } else {
            Some(SelectedItem::Crate(index))
        };
    }
}
