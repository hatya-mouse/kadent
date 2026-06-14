mod new_project_dialog;
mod project_list;
mod splash_controls;
pub(crate) mod state;

use crate::{
    consts::RECENT_PROJCETS_MAX_NUM,
    core::metadata::ProjectMeta,
    storage::{
        app_state::save_recent_projects,
        project::{get_project_dir, init_kasl_nodes, load_project},
    },
    ui::{theme, workspaces::splash::state::SplashUiState},
    utils::version_string,
};
use eframe::egui;
use kadent_engine::{data_types::AudioContext, mixer::Project};
use std::path::{Path, PathBuf};

const PROJECT_LIST_THRESHOLD: f32 = 240.0;

/// The splash screen of Kadent.
pub struct SplashUi {
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

/// A struct that contains the data passed to the editor UI.
pub struct EditorTransition {
    pub project_path: PathBuf,
    pub audio_ctx: AudioContext,
    pub project: Project,
    pub project_meta: ProjectMeta,
}

impl SplashUi {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<EditorTransition> {
        let mut transition = None;

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
                        transition = Some(t);
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
                            transition = Some(t);
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
            transition = Some(t);
        }

        transition
    }

    /// Opens the project file at the given path, returning the transition data if successful.
    fn open_project(&self, project_path: PathBuf) -> Option<EditorTransition> {
        // Store the project to recent projects
        self.add_and_store_recent_projects(&project_path);

        // Load the project and pass the data to the editor UI
        match load_project(&project_path) {
            Ok(mut proj_res) => match ProjectMeta::from_load_res(&proj_res) {
                Ok(project_meta) => {
                    let project_dir = get_project_dir(&project_path);
                    init_kasl_nodes(
                        &mut proj_res.project,
                        &project_meta.kasl_search_paths,
                        &project_dir,
                    );

                    let audio_ctx = proj_res.project.audio_ctx.clone();
                    Some(EditorTransition {
                        project_path,
                        audio_ctx,
                        project: proj_res.project,
                        project_meta,
                    })
                }
                Err(e) => {
                    eprintln!("Failed to extract project metadata: {:?}", e);
                    None
                }
            },
            Err(e) => {
                eprintln!("Failed to load project: {:?}", e);
                None
            }
        }
    }

    fn add_and_store_recent_projects(&self, project_path: &Path) {
        let mut project_paths: Vec<PathBuf> = self
            .splash_state
            .recent_projects
            .lock()
            .unwrap()
            .iter()
            .map(|proj| proj.path.clone())
            .collect();

        // Add the project at the first
        // If the same project already exists, remove the existing one and add new one at the first
        let project_path_buf = project_path.to_path_buf();
        if let Some(existing_index) = project_paths
            .iter()
            .position(|path_buf| path_buf == &project_path_buf)
        {
            project_paths.remove(existing_index);
        }
        project_paths.insert(0, project_path_buf);
        // Limit the number of recent projcets
        project_paths.truncate(RECENT_PROJCETS_MAX_NUM);
        project_paths.shrink_to_fit();

        // Save the paths to the disk
        save_recent_projects(&project_paths);
    }
}
