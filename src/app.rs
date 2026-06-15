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
        // Show the splash screen if we're in the splash state
        // Toggle to the editor UI if the splash screen returns an editor context
        match self {
            KadentApp::Splash(splash) => {
                let editor_ctx = egui::CentralPanel::default()
                    .frame(
                        egui::Frame::new()
                            .fill(theme::primary_bg(ui.visuals().dark_mode))
                            .inner_margin(0),
                    )
                    .show_inside(ui, |ui| splash.ui(ui))
                    .inner;

                // If the splash screen returned an editor context, switch to the editor UI
                if let Some(editor_ctx) = editor_ctx {
                    *self = KadentApp::Editor(Box::new(EditorUi::new(editor_ctx)));
                }
            }
            KadentApp::Editor(app) => {
                // If we're in the editor state, just show the editor UI
                app.editor_ui(ui, frame);
            }
        }
    }
}
