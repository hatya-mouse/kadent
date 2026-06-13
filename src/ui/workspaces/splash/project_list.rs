use crate::ui::workspaces::{EditorTransition, SplashUi};
use eframe::egui;

impl SplashUi {
    pub(super) fn project_list(&mut self, ui: &mut egui::Ui) -> Option<EditorTransition> {
        let mut selected_path = None;

        egui::ScrollArea::vertical()
            .content_margin(egui::Margin::same(6))
            .show(ui, |ui| {
                let Ok(recent_projects) = self.splash_state.recent_projects.lock() else {
                    return;
                };

                for project in recent_projects.iter() {
                    let frame_response = egui::Frame::new()
                        .show(ui, |ui| {
                            ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 4.0);
                            // Top: Show filename
                            ui.label(egui::RichText::new(&project.name).strong().size(14.0));
                            // Bottom: Show full path in smaller, weaker text
                            let path_label = egui::Label::new(
                                egui::RichText::new(&project.path_str).small().weak(),
                            )
                            .wrap_mode(egui::TextWrapMode::Wrap); // Forces text to wrap onto new lines
                            ui.add(path_label);
                        })
                        .response;

                    let response = ui.interact(
                        frame_response.rect,
                        ui.id().with(&project.path_str),
                        egui::Sense::click(),
                    );

                    if response.hovered() {
                        ui.painter().rect_filled(
                            response.rect,
                            egui::CornerRadius::same(4),
                            egui::Color32::from_white_alpha(10),
                        );
                    }

                    if response.clicked() {
                        selected_path = Some(project.path.clone());
                    }
                }
            });

        // Open the project is any is selected
        if let Some(path) = selected_path {
            self.open_project(path)
        } else {
            None
        }
    }
}
