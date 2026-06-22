use crate::{
    core::project_ctx::EditorContext,
    storage::project::{open_project_to_ctx, save_project},
    ui::{
        components::{icon_button::toolbar_icon_button, toolbar_group::toolbar_group},
        workspaces::EditorUi,
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
                match save_project(
                    &self.proj_ctx.project_path,
                    &self.proj_ctx.project,
                    &self.proj_ctx.project_meta,
                ) {
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
                self.handle_open_project(proj_path_option);
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

    fn handle_open_project(&mut self, proj_path_option: Option<std::path::PathBuf>) {
        let Some(proj_path) = proj_path_option else {
            return;
        };
        let Some(editor_ctx) = open_project_to_ctx(proj_path) else {
            return;
        };

        self.set_editor_ctx(editor_ctx);
    }

    pub(crate) fn set_editor_ctx(&mut self, editor_ctx: EditorContext) {
        self.proj_ctx = editor_ctx.proj_ctx;

        // Seek to the start of the project after loading
        let command = AudioCommand::Seek(self.proj_ctx.project.range_start);
        if self
            .thread_handle
            .audio_command_tx
            .send(command.clone())
            .is_err()
        {
            self.errors.push(AudioError::CommandFailed(command));
        }

        // Notify the audio thread of the project change
        self.modified_project();
    }
}
