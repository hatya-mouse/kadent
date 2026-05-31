use crate::{
    load_write::{init_kasl_nodes, load_project_from_dir},
    metadata::ProjectMeta,
    ui::EditorUi,
};
use eframe::egui;
use kreniq_engine::{
    data_types::{AudioContext, Beats},
    mixer::Project,
};
use std::path::PathBuf;

pub struct SplashUi;

pub enum SplashTransition {
    NewProject {
        project_dir: PathBuf,
        audio_ctx: AudioContext,
        project: Project,
        project_meta: ProjectMeta,
    },
    OpenProject {
        project_dir: PathBuf,
        audio_ctx: AudioContext,
        project: Project,
        project_meta: ProjectMeta,
    },
}

impl SplashUi {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<SplashTransition> {
        ui.vertical_centered(|ui| {
            ui.add_space(ui.available_height() / 3.0);
            ui.heading("Kreniq");
            ui.add_space(16.0);

            if ui.button("New Project").clicked()
                && let Some(project_dir) = rfd::FileDialog::new().save_file()
            {
                let audio_ctx = AudioContext {
                    channels: 2,
                    sample_rate: 44100,
                    buffer_size: 512,
                    max_voices: 32,
                };
                let mut project = Project::new(audio_ctx.clone(), 120.0, Beats(0.0), Beats(8.0));
                let mut project_meta = ProjectMeta {
                    kasl_search_paths: EditorUi::system_kasl_search_paths(),
                    ..Default::default()
                };
                EditorUi::setup_project(&project_dir, &mut project, &mut project_meta, &audio_ctx);
                return Some(SplashTransition::NewProject {
                    project_dir,
                    audio_ctx,
                    project,
                    project_meta,
                });
            }

            if ui.button("Open Project").clicked()
                && let Some(project_dir) = rfd::FileDialog::new().pick_folder()
            {
                match load_project_from_dir(&project_dir) {
                    Ok(mut proj_res) => match ProjectMeta::from_load_res(&proj_res) {
                        Ok(project_meta) => {
                            init_kasl_nodes(
                                &mut proj_res.project,
                                &project_meta.kasl_search_paths,
                                &project_dir,
                            );

                            let audio_ctx = proj_res.project.audio_ctx.clone();
                            return Some(SplashTransition::OpenProject {
                                project_dir,
                                audio_ctx,
                                project: proj_res.project,
                                project_meta,
                            });
                        }
                        Err(e) => eprintln!("Failed to extract project metadata: {:?}", e),
                    },
                    Err(e) => eprintln!("Failed to load project: {:?}", e),
                }
            }

            None
        })
        .inner
    }
}
