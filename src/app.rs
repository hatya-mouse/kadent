use crate::{
    consts::PROJECT_FILE_EXTENSION,
    core::project_ctx::ProjectContext,
    storage::{
        app_state::{AppPreferences, load_preferences},
        project::open_project_to_ctx,
    },
    ui::{EditorUi, SplashUi, theme},
};
use eframe::{self, egui};
use std::path::PathBuf;

pub(crate) struct KadentApp {
    pub(crate) state: AppState,
    pub(crate) preferences: AppPreferences,
}

pub(crate) enum AppState {
    Splash(Box<SplashUi>),
    Editor(Box<EditorUi>),
}

enum GetDroppedFileResult {
    ProjectData(Box<ProjectContext>),
    AudioFile(PathBuf),
}

pub(crate) enum AppTransition {
    ToEditor(Box<ProjectContext>),
}

impl KadentApp {
    pub(crate) fn new(cc: &eframe::CreationContext, initial_project: Option<PathBuf>) -> Self {
        // Load the preferences
        let preferences = load_preferences();

        egui_extras::install_image_loaders(&cc.egui_ctx);
        Self::setup_fonts(&cc.egui_ctx);
        Self::base_style(&cc.egui_ctx);

        if let Some(initial_project) = initial_project
            && let Some(editor_ctx) = open_project_to_ctx(initial_project, &preferences)
        {
            return KadentApp {
                state: AppState::Editor(Box::new(EditorUi::new(editor_ctx))),
                preferences,
            };
        }
        KadentApp {
            state: AppState::Splash(Box::default()),
            preferences,
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
            if let Some(file) = input.raw.dropped_files.first() {
                let path = file.path();
                if let Some(extension) = path.extension() {
                    if extension == PROJECT_FILE_EXTENSION {
                        return open_project_to_ctx(path.to_path_buf(), &self.preferences)
                            .map(|ctx| GetDroppedFileResult::ProjectData(Box::new(ctx)));
                    } else if extension == "wav" {
                        return Some(GetDroppedFileResult::AudioFile(path.to_path_buf()));
                    }
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
                if let Some(transition) = egui::CentralPanel::default()
                    .frame(
                        egui::Frame::new()
                            .fill(theme::primary_bg(ui.visuals().dark_mode))
                            .inner_margin(0),
                    )
                    .show(ui, |ui| splash.ui(ui, &mut self.preferences))
                    .inner
                    && let AppTransition::ToEditor(editor_ctx) = transition
                {
                    self.state = AppState::Editor(Box::new(EditorUi::new(*editor_ctx)));
                }

                if let Some(GetDroppedFileResult::ProjectData(ctx)) = dropped_file {
                    self.state = AppState::Editor(Box::new(EditorUi::new(*ctx)));
                }
            }
            AppState::Editor(editor) => {
                // If we're in the editor state, just show the editor UI
                editor.ui(ui, &self.preferences);

                if let Some(GetDroppedFileResult::AudioFile(dropped_path)) = dropped_file {
                    editor.audio_dropped(dropped_path);
                } else if let Some(hovered_path) = hovered_file {
                    editor.audio_hovered(hovered_path);
                }
            }
        }
    }
}
