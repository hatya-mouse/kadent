use crate::{
    components::icon_button::toolbar_icon_button,
    load_write::{init_kasl_nodes, load_project_from_dir, save_project_to_dir},
    metadata::ProjectMeta,
    ui::{EditorUi, editor::toolbar::toolbar_group::toolbar_group},
};
use eframe::egui;
use kreniq_engine::thread::{AudioCommand, AudioError};

impl EditorUi {
    pub(super) fn file_control(&mut self, ui: &mut egui::Ui) {
        toolbar_group(ui, |ui| {
            if toolbar_icon_button(
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

            if toolbar_icon_button(
                ui,
                egui::Image::new(egui::include_image!("../../../../assets/icons/open.svg")),
            )
            .clicked()
            {
                let proj_path_option = rfd::FileDialog::new().pick_folder();

                if let Some(proj_path) = proj_path_option {
                    match load_project_from_dir(&proj_path) {
                        Ok(proj_res) => match ProjectMeta::from_load_res(&proj_res) {
                            Ok(project_meta) => {
                                self.project_meta = project_meta;
                                self.project = proj_res.project;

                                init_kasl_nodes(
                                    &mut self.project,
                                    &self.project_meta.kasl_search_paths,
                                    &proj_path,
                                );

                                self.project_dir = proj_path;

                                let command = AudioCommand::Seek(self.project.range_start);
                                if self
                                    .thread_handle
                                    .audio_command_tx
                                    .send(command.clone())
                                    .is_err()
                                {
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

            if toolbar_icon_button(
                ui,
                egui::Image::new(egui::include_image!(
                    "../../../../assets/icons/waveform.svg"
                )),
            )
            .clicked()
            {
                let export_path = rfd::FileDialog::new()
                    .add_filter("WAV file", &["wav"])
                    .save_file();

                if let Some(path) = export_path {
                    self.export_project(&path);
                }
            }
        });
    }
}
