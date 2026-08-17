use crate::{
    consts::PROJECT_FILE_EXTENSION,
    core::project_ctx::EditorContext,
    storage::project::open_project_to_ctx,
    ui::{EditorState, SplashUi, theme},
};
use eframe::{self, egui};
use std::path::PathBuf;

pub struct KadentApp {
    pub state: AppState,
}

pub enum AppState {
    Splash(Box<SplashUi>),
    Editor(Box<EditorState>),
}

enum GetDroppedFileResult {
    Project(Box<EditorContext>),
    AudioFile(PathBuf),
}

impl KadentApp {
    pub fn new(cc: &eframe::CreationContext, initial_project: Option<PathBuf>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Self::setup_fonts(&cc.egui_ctx);
        Self::base_style(&cc.egui_ctx);

        if let Some(initial_project) = initial_project
            && let Some(editor_ctx) = open_project_to_ctx(initial_project)
        {
            return KadentApp {
                state: AppState::Editor(Box::new(EditorState::new(editor_ctx))),
            };
        }
        KadentApp {
            state: AppState::Splash(Box::default()),
        }
    }

    pub(crate) fn base_style(ctx: &egui::Context) {
        ctx.all_styles_mut(|style| {
            style.interaction.selectable_labels = false;
            style.visuals.window_shadow = egui::Shadow::NONE;
            style.visuals.popup_shadow = egui::Shadow::NONE;
        });
    }

    fn get_dropped_file(&self, ui: &egui::Ui) -> Option<GetDroppedFileResult> {
        ui.ctx().input(|input| {
            if let Some(file) = input.raw.dropped_files.first()
                && let Some(path) = &file.path
                && let Some(extension) = path.extension()
            {
                if extension == PROJECT_FILE_EXTENSION {
                    return open_project_to_ctx(path.clone())
                        .map(|ctx| GetDroppedFileResult::Project(Box::new(ctx)));
                } else if extension == "wav" {
                    return Some(GetDroppedFileResult::AudioFile(path.clone()));
                }
            }

            None
        })
    }

    fn get_hovered_audio_file(&self, ui: &egui::Ui) -> Option<PathBuf> {
        ui.ctx().input(|input| {
            if let Some(file) = input.raw.hovered_files.first()
                && let Some(path) = &file.path
            {
                return Some(path.clone());
            }

            None
        })
    }
}

impl eframe::App for KadentApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let dropped_file = self.get_dropped_file(ui);
        let hovered_file = self.get_hovered_audio_file(ui);

        // Show the splash screen if we're in the splash state
        // Toggle to the editor UI if the splash screen returns an editor context
        match &mut self.state {
            AppState::Splash(splash) => {
                if let Some(editor_ctx) = egui::CentralPanel::default()
                    .frame(
                        egui::Frame::new()
                            .fill(theme::primary_bg(ui.visuals().dark_mode))
                            .inner_margin(0),
                    )
                    .show_inside(ui, |ui| splash.splash_ui(ui))
                    .inner
                {
                    // If the splash screen returned an editor context, switch to the editor UI
                    self.state = AppState::Editor(Box::new(EditorState::new(editor_ctx)));
                }

                if let Some(GetDroppedFileResult::Project(ctx)) = dropped_file {
                    self.state = AppState::Editor(Box::new(EditorState::new(*ctx)));
                }
            }
            AppState::Editor(editor) => {
                // If we're in the editor state, just show the editor UI
                editor.editor_ui(ui);

                if let Some(GetDroppedFileResult::AudioFile(dropped_path)) = dropped_file {
                    editor.audio_dropped(dropped_path);
                } else if let Some(hovered_path) = hovered_file {
                    editor.audio_hovered(hovered_path);
                }
            }
        }
    }
}
