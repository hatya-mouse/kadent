use crate::{
    theme,
    ui::{EditorUi, SplashTransition, SplashUi},
};
use eframe::{self, egui};

pub enum KrenicApp {
    Splash(Box<SplashUi>),
    Editor(Box<EditorUi>),
}

impl KrenicApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Self::setup_fonts(&cc.egui_ctx);
        Self::base_style(&cc.egui_ctx);
        KrenicApp::Splash(Box::new(SplashUi))
    }

    pub(crate) fn base_style(ctx: &egui::Context) {
        ctx.global_style_mut(|style| {
            style.interaction.selectable_labels = false;
            style.visuals.window_shadow = egui::Shadow::NONE;
            style.visuals.popup_shadow = egui::Shadow::NONE;
        });
    }
}

impl eframe::App for KrenicApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // Compute any splash→editor transition before mutating self.
        let transition = if let KrenicApp::Splash(splash) = self {
            egui::CentralPanel::default()
                .frame(
                    egui::Frame::new()
                        .fill(theme::primary_bg(ui.visuals().dark_mode))
                        .inner_margin(0),
                )
                .show_inside(ui, |ui| splash.ui(ui))
                .inner
        } else if let KrenicApp::Editor(app) = self {
            app.editor_ui(ui, frame);
            None
        } else {
            None
        };

        if let Some(transition) = transition {
            let (project_dir, audio_ctx, project, project_meta) = match transition {
                SplashTransition::NewProject {
                    project_dir,
                    audio_ctx,
                    project,
                    project_meta,
                }
                | SplashTransition::OpenProject {
                    project_dir,
                    audio_ctx,
                    project,
                    project_meta,
                } => (project_dir, audio_ctx, project, project_meta),
            };
            *self = KrenicApp::Editor(Box::new(EditorUi::new(
                project_dir,
                audio_ctx,
                project,
                project_meta,
            )));
        }
    }
}
