mod crate_licenses;
mod dep_detail;
mod dep_list;

use crate::{
    consts::{MAX_SIDEBAR_WIDTH, MIN_SIDEBAR_WIDTH},
    ui::{
        components::v_splitter::VSplitter,
        splash::dialog::{
            SplashDialogState,
            license_dialog::crate_licenses::{LicenseDocument, load_crate_licenses},
        },
    },
};
use eframe::egui;

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

        ui.horizontal(|ui| {
            dialog_state.dep_list(ui);

            VSplitter::new(&mut dialog_state.dep_list_width)
                .with_min(MIN_SIDEBAR_WIDTH)
                .with_max(MAX_SIDEBAR_WIDTH)
                .show(ui);

            dialog_state.dep_detail(ui);
        });
    }
}
