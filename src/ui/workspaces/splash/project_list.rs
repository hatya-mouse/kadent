use crate::{
    fonts::RichTextExt,
    ui::workspaces::{EditorTransition, SplashUi},
};
use eframe::egui;

const CONTENT_MARGIN: i8 = 8;

impl SplashUi {
    pub(super) fn project_list(&mut self, ui: &mut egui::Ui) -> Option<EditorTransition> {
        let mut selected_path = None;

        egui::ScrollArea::vertical()
            .content_margin(egui::Margin::same(CONTENT_MARGIN))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let item_width = (ui.available_width() - CONTENT_MARGIN as f32 * 2.0).max(0.0);
                let Ok(recent_projects) = self.splash_state.recent_projects.lock() else {
                    return;
                };

                for project in recent_projects.iter() {
                    let frame_response = egui::Frame::new()
                        .inner_margin(egui::Margin::symmetric(6, 4))
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

                    if response.hovered() {
                        ui.painter().rect_filled(
                            response.rect,
                            egui::CornerRadius::same(6),
                            if ui.visuals().dark_mode {
                                egui::Color32::from_white_alpha(10)
                            } else {
                                egui::Color32::from_black_alpha(10)
                            },
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
