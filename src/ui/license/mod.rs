mod crate_licenses;
mod dep_detail;
mod dep_list;
mod font_licenses;

use crate::{
    consts::{MAX_SIDEBAR_WIDTH, MIN_SIDEBAR_WIDTH},
    ui::components::v_splitter::VSplitter,
};
use eframe::egui;

struct LicenseItem {
    name: String,
    id: String,
    text: String,
}

struct DependencyItem {
    name: String,
    version: String,
    authors: Vec<String>,
    description: String,
    license_id: String,
}

enum SelectedItem {
    Font(usize),
    Crate(usize),
}

pub(crate) struct LicenseUi {
    /// The width of the left dependency list panel.
    dep_list_width: f32,
    /// The loaded licenses.
    licenses: Vec<LicenseItem>,
    /// The loaded font dependency data.
    fonts: Vec<DependencyItem>,
    /// The loaded crate dependency data.
    crates: Vec<DependencyItem>,
    /// The selected dependency index.
    selected_dep: Option<SelectedItem>,
}

impl Default for LicenseUi {
    fn default() -> Self {
        Self {
            dep_list_width: 200.0,
            licenses: Vec::new(),
            fonts: Vec::new(),
            crates: Vec::new(),
            selected_dep: None,
        }
    }
}

impl LicenseUi {
    pub(crate) fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            self.dep_list(ui);

            VSplitter::new(&mut self.dep_list_width)
                .with_min(MIN_SIDEBAR_WIDTH)
                .with_max(MAX_SIDEBAR_WIDTH)
                .show(ui);

            self.dep_detail(ui);
        });
    }
}
