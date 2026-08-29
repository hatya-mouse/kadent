use crate::{
    fonts::RichTextExt,
    ui::{
        components::not_available_text::not_available_text,
        splash::dialog::{
            LicenseDialogState,
            license_dialog::{LICENSE_MARGIN, SelectedItem, crate_licenses::DependencyItem},
        },
        theme,
    },
};
use eframe::egui;

const ROW_HEIGHT: f32 = 24.0;

impl LicenseDialogState {
    pub(super) fn dep_list(&mut self, ui: &mut egui::Ui, width: f32) {
        ui.scope(|ui| {
            let Ok(document) = &self.document else {
                not_available_text(ui, "Failed to load licenses.");
                return;
            };

            *ui.style_mut() = theme::menu_style(ui);
            let total_rows = document.fonts.len() + document.crates.len() + 2;

            let corner_radius = ui.style().visuals.window_corner_radius;
            ui.allocate_ui(egui::Vec2::new(width, ui.available_height()), |ui| {
                egui::Frame::new()
                    .fill(theme::secondary_bg(ui.visuals().dark_mode))
                    .corner_radius(egui::CornerRadius {
                        nw: 0,
                        ne: 0,
                        sw: corner_radius.sw,
                        se: 0,
                    })
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .id_salt("dep_list")
                            .content_margin(LICENSE_MARGIN)
                            .show_rows(ui, ROW_HEIGHT, total_rows, |ui, row_range| {
                                ui.vertical(|ui| {
                                    for row in row_range {
                                        if row == 0 {
                                            ui.label(
                                                egui::RichText::new("Fonts").bold().size(16.0),
                                            );
                                        } else if row <= document.fonts.len() {
                                            let index = row - 1;
                                            let font_item = &document.fonts[index];
                                            dep_list_item(
                                                ui,
                                                &mut self.selected_dep,
                                                font_item,
                                                index,
                                                true,
                                            );
                                        } else if row == document.fonts.len() + 1 {
                                            ui.label(
                                                egui::RichText::new("Crates").bold().size(16.0),
                                            );
                                        } else {
                                            let index = row - document.fonts.len() - 2;
                                            let crate_item = &document.crates[index];
                                            dep_list_item(
                                                ui,
                                                &mut self.selected_dep,
                                                crate_item,
                                                index,
                                                false,
                                            );
                                        }
                                    }
                                });
                            });
                    });
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

    let response = ui.add_sized(
        egui::vec2(ui.available_width(), ROW_HEIGHT),
        egui::Button::selectable(is_selected, &item.name)
            .right_text("")
            .truncate(),
    );
    if response.clicked() {
        *selected_dep = if is_font {
            Some(SelectedItem::Font(index))
        } else {
            Some(SelectedItem::Crate(index))
        };
    }
}
