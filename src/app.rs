use crate::{
    storage::project::open_project_to_ctx,
    ui::{
        theme,
        workspaces::{EditorUi, SplashUi},
    },
};
use eframe::{self, egui};
use std::path::PathBuf;

pub enum KadentApp {
    Splash(Box<SplashUi>),
    Editor(Box<EditorUi>),
}

impl KadentApp {
    pub fn new(cc: &eframe::CreationContext, initial_project: Option<PathBuf>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Self::setup_fonts(&cc.egui_ctx);
        Self::base_style(&cc.egui_ctx);

        if let Some(initial_project) = initial_project
            && let Some(editor_ctx) = open_project_to_ctx(initial_project)
        {
            return KadentApp::Editor(Box::new(EditorUi::new(editor_ctx)));
        }
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
        let editor_ctx = if let KadentApp::Splash(splash) = self {
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

        if let Some(editor_ctx) = editor_ctx {
            *self = KadentApp::Editor(Box::new(EditorUi::new(editor_ctx)));
        }
    }
}
