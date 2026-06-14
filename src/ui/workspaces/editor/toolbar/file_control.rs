use crate::{
    core::metadata::ProjectMeta,
    storage::project::{get_project_dir, init_kasl_nodes, load_project, save_project},
    ui::{
        components::icon_button::toolbar_icon_button,
        workspaces::{EditorUi, editor::toolbar::toolbar_group::toolbar_group},
    },
};
use eframe::egui;
use kadent_engine::thread::{AudioCommand, AudioError};

impl EditorUi {
    pub(super) fn file_control(&mut self, ui: &mut egui::Ui) {
        toolbar_group(ui, |ui| {
            if toolbar_icon_button(
                ui,
                egui::Image::new(egui::include_image!("../../../../../assets/icons/save.svg")),
            )
            .clicked()
            {
                match save_project(&self.project_path, &self.project, &self.project_meta) {
                    Ok(()) => (),
                    Err(e) => {
                        eprintln!("Failed to save project: {:?}", e);
                    }
                }
            }

            if toolbar_icon_button(
                ui,
                egui::Image::new(egui::include_image!("../../../../../assets/icons/open.svg")),
            )
            .clicked()
            {
                let proj_path_option = rfd::FileDialog::new().pick_file();
                self.handle_load_project(proj_path_option);
            }

            if toolbar_icon_button(
                ui,
                egui::Image::new(egui::include_image!(
                    "../../../../../assets/icons/waveform.svg"
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

    fn handle_load_project(&mut self, proj_path_option: Option<std::path::PathBuf>) {
        if let Some(proj_path) = proj_path_option {
            match load_project(&proj_path) {
                Ok(proj_res) => match ProjectMeta::from_load_res(&proj_res) {
                    Ok(project_meta) => {
                        self.project_meta = project_meta;
                        self.project = proj_res.project;
                        self.project_path = proj_path;
                        let project_dir = get_project_dir(&self.project_path);

                        init_kasl_nodes(
                            &mut self.project,
                            &self.project_meta.kasl_search_paths,
                            &project_dir,
                        );

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
}
