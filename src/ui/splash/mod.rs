mod new_project_dialog;
mod project_list;
mod splash_controls;
pub(crate) mod state;

use crate::{
    core::project_ctx::ProjectContext,
    ui::{splash::state::SplashUiState, theme},
    utils::version_string,
};
use eframe::egui;

const PROJECT_LIST_THRESHOLD: f32 = 240.0;

/// The splash screen of Kadent.
pub(crate) struct SplashUi {
    /// The current splash UI state.
    splash_state: SplashUiState,
    /// The version text displayed in the splash screen.
    version_string: String,
}

impl Default for SplashUi {
    fn default() -> Self {
        Self {
            version_string: version_string(),
            splash_state: SplashUiState::default(),
        }
    }
}

impl SplashUi {
    pub(crate) fn splash_ui(&mut self, ui: &mut egui::Ui) -> Option<ProjectContext> {
        let mut ctx = None;

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            let full_width = ui.available_width();
            let full_height = ui.available_height();
            ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);

            let base_control_width = (full_width * 0.4).max(400.0).min(full_width);
            let project_list_width = full_width - base_control_width;
            let show_project_list = full_width - base_control_width >= PROJECT_LIST_THRESHOLD;
            let splash_control_width = if show_project_list {
                base_control_width
            } else {
                full_width
            };

            // If the remaining width is smaller than that threshold, collapse the project list and show only the splash controls
            ui.allocate_ui_with_layout(
                egui::vec2(splash_control_width, full_height),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    if let Some(t) = self.splash_controls(ui) {
                        ctx = Some(t);
                    }
                },
            );

            if show_project_list {
                let separator_x = ui.cursor().min.x;

                ui.allocate_ui_with_layout(
                    egui::vec2(project_list_width, full_height),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        if let Some(t) = self.project_list(ui) {
                            ctx = Some(t);
                        }
                    },
                );

                let rect = ui.min_rect();
                ui.painter().line_segment(
                    [
                        egui::pos2(separator_x, rect.min.y),
                        egui::pos2(separator_x, rect.max.y),
                    ],
                    theme::border(ui.visuals().dark_mode),
                );
            }
        });

        if let Some(t) = self.new_project_dialog(ui) {
            ctx = Some(t);
        }

        ctx
    }
}
