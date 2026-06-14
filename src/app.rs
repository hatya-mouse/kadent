use crate::ui::{
    theme,
    workspaces::{EditorUi, SplashUi},
};
use eframe::{self, egui};

pub enum KadentApp {
    Splash(Box<SplashUi>),
    Editor(Box<EditorUi>),
}

impl KadentApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Self::setup_fonts(&cc.egui_ctx);
        Self::base_style(&cc.egui_ctx);
        KadentApp::Splash(Box::default())
    }

    pub(crate) fn base_style(ctx: &egui::Context) {
        ctx.all_styles_mut(|style| {
            style.interaction.selectable_labels = false;
            style.visuals.window_shadow = egui::Shadow::NONE;
            style.visuals.popup_shadow = egui::Shadow::NONE;
        });
    }
}

impl eframe::App for KadentApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // Compute any splash→editor transition before mutating self.
        let transition = if let KadentApp::Splash(splash) = self {
            egui::CentralPanel::default()
                .frame(
                    egui::Frame::new()
                        .fill(theme::primary_bg(ui.visuals().dark_mode))
                        .inner_margin(0),
                )
                .show_inside(ui, |ui| splash.ui(ui))
                .inner
        } else if let KadentApp::Editor(app) = self {
            app.editor_ui(ui, frame);
            None
        } else {
            None
        };

        if let Some(transition) = transition {
            *self = KadentApp::Editor(Box::new(EditorUi::new(
                transition.project_path,
                transition.audio_ctx,
                transition.project,
                transition.project_meta,
            )));
        }
    }
}
