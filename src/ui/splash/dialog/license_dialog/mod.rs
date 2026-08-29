mod crate_licenses;
mod dep_detail;
mod dep_list;

use crate::{
    consts::{MAX_SIDEBAR_WIDTH, MIN_SIDEBAR_WIDTH},
    ui::{
        components::{dialog::dialog, v_splitter::VSplitter},
        splash::dialog::{
            SplashDialogState,
            license_dialog::crate_licenses::{LicenseDocument, load_crate_licenses},
        },
    },
};
use eframe::egui;

const LICENSE_MARGIN: egui::Margin = egui::Margin::symmetric(8, 6);

enum SelectedItem {
    Font(usize),
    Crate(usize),
}

pub(crate) struct LicenseDialogState {
    /// The width of the left dependency list panel.
    dep_list_width: f32,
    /// The loaded licenses data.
    document: std::io::Result<LicenseDocument>,
    /// The selected dependency index.
    selected_dep: Option<SelectedItem>,
}

impl Default for LicenseDialogState {
    fn default() -> Self {
        Self {
            dep_list_width: 200.0,
            document: load_crate_licenses(),
            selected_dep: None,
        }
    }
}

impl SplashDialogState {
    pub(crate) fn license_dialog(&mut self, ui: &mut egui::Ui) {
        let SplashDialogState::License(dialog_state) = self else {
            return;
        };
        let viewport_size = ui.viewport_rect().size();
        let dialog_size = egui::Vec2::new(viewport_size.x * 0.8, viewport_size.y * 0.8);

        let modal = dialog(ui, "Acknowledgements", 0, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
                ui.set_max_size(dialog_size);
                ui.set_min_size(dialog_size);

                dialog_state.dep_list(ui, dialog_state.dep_list_width);

                VSplitter::new(&mut dialog_state.dep_list_width)
                    .with_min(MIN_SIDEBAR_WIDTH)
                    .with_max(MAX_SIDEBAR_WIDTH.min(dialog_size.x * 0.5))
                    .show(ui);

                dialog_state.dep_detail(ui);
            });
        });

        if modal.should_close() {
            *self = SplashDialogState::None;
        }
    }
}
