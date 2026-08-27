use crate::ui::{
    EditorState,
    components::{toolbar_button::toolbar_icon_button, toolbar_group::toolbar_group},
    editor::actions::EditorAction,
};
use eframe::egui;

pub(super) fn file_control(ui: &mut egui::Ui, state: &mut EditorState) {
    toolbar_group(ui, |ui| {
        if toolbar_icon_button(
            ui,
            egui::Image::new(egui::include_image!("../../../../assets/icons/save.svg")),
        )
        .clicked()
        {
            state.actions.push_action(EditorAction::SaveAll);
        }

        if toolbar_icon_button(
            ui,
            egui::Image::new(egui::include_image!("../../../../assets/icons/open.svg")),
        )
        .clicked()
            && let Some(proj_path) = rfd::FileDialog::new().pick_file()
        {
            state
                .actions
                .push_action(EditorAction::OpenProject(proj_path));
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
                state.actions.push_action(EditorAction::ExportProject(path));
            }
        }
    });
}
