use crate::{
    app::KnodiqApp,
    components::icon_button::icon_button,
    load_write::{load_project_from_dir, save_project_to_dir},
    metadata::ProjectMeta,
    ui::editor::toolbar::toolbar_group::toolbar_group,
};
use eframe::egui;
use knodiq_engine::audio_thread::{AudioCommand, error::AudioError};

impl KnodiqApp {
    pub(super) fn file_control(&mut self, ui: &mut egui::Ui) {
        toolbar_group(ui, |ui| {
            if icon_button(
                ui,
                egui::Image::new(egui::include_image!("../../../../assets/icons/save.svg")),
            )
            .clicked()
            {
                let files = rfd::FileDialog::new().save_file();

                if let Some(path) = files {
                    match save_project_to_dir(&path, &self.project, &self.project_meta) {
                        Ok(()) => (),
                        Err(e) => {
                            eprintln!("Failed to save project: {:?}", e);
                        }
                    }
                }
            }

            if icon_button(
                ui,
                egui::Image::new(egui::include_image!("../../../../assets/icons/open.svg")),
            )
            .clicked()
            {
                let files = rfd::FileDialog::new().pick_folder();

                if let Some(path) = files {
                    match load_project_from_dir(&path) {
                        Ok(mut proj_res) => match ProjectMeta::from_load_res(&proj_res) {
                            Ok(project_meta) => {
                                KnodiqApp::apply_kasl_search_paths(
                                    &mut proj_res.project,
                                    &project_meta.kasl_search_paths,
                                );

                                self.project_meta = project_meta;
                                self.project = proj_res.project;

                                let command = AudioCommand::Seek(self.project.range_start);
                                if self.thread_handle.command_tx.send(command.clone()).is_err() {
                                    self.errors.push(AudioError::CommandFailed(command));
                                }

                                self.modified_project();
                            }
                            Err(e) => {
                                eprintln!("Failed to extract project metadata: {:?}", e);
                            }
                        },
                        Err(e) => {
                            eprintln!("Failed to load project: {:?}", e);
                        }
                    }
                }
            }
        });
    }
}
