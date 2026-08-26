use crate::{
    core::project_ctx::ProjectContext,
    fonts::RichTextExt,
    storage::{app_state::AppPreferences, project::open_project_to_ctx},
    ui::{SplashUi, components::not_available_text::not_available_text, theme},
};
use eframe::egui;

const CONTENT_MARGIN: i8 = 12;

impl SplashUi {
    pub(super) fn project_list(
        &mut self,
        ui: &mut egui::Ui,
        preferences: &AppPreferences,
    ) -> Option<ProjectContext> {
        let mut selected_path = None;

        egui::ScrollArea::vertical()
            .id_salt("recent_projects")
            .content_margin(egui::Margin::same(CONTENT_MARGIN))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let item_width = (ui.available_width() - CONTENT_MARGIN as f32 * 2.0).max(0.0);
                let Ok(recent_projects) = self.splash_state.recent_projects.lock() else {
                    not_available_text(ui, "Failed to load recent projects");
                    return;
                };

                if recent_projects.is_empty() {
                    not_available_text(ui, "No recent projects found");
                    return;
                }

                for project in recent_projects.iter() {
                    let frame_response = egui::Frame::new()
                        .inner_margin(egui::Margin::symmetric(10, 8))
                        .show(ui, |ui| {
                            ui.set_min_width(item_width);
                            ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 4.0);

                            // Top: Show filename
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(&project.name)
                                        .bold()
                                        .strong()
                                        .size(14.0),
                                )
                                .selectable(false),
                            );
                            // Bottom: Show full path in smaller, weaker text
                            ui.add(
                                egui::Label::new(egui::RichText::new(&project.path_str).weak())
                                    .wrap_mode(egui::TextWrapMode::Wrap)
                                    .selectable(false),
                            );
                        })
                        .response;

                    let response = ui.interact(
                        frame_response.rect,
                        ui.id().with(&project.path_str),
                        egui::Sense::click(),
                    );

                    if response.is_pointer_button_down_on() {
                        ui.painter().rect_filled(
                            response.rect,
                            egui::CornerRadius::same(6),
                            theme::card_button_pressed(ui.visuals().dark_mode),
                        );
                    } else if response.hovered() {
                        ui.painter().rect_filled(
                            response.rect,
                            egui::CornerRadius::same(6),
                            theme::card_button_hovered(ui.visuals().dark_mode),
                        );
                    }

                    if response.clicked() {
                        selected_path = Some(project.path.clone());
                    }
                }
            });

        // Open the project is any is selected
        if let Some(path) = selected_path {
            open_project_to_ctx(path, preferences)
        } else {
            None
        }
    }
}
