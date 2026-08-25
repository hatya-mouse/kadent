use crate::{
    fonts::RichTextExt,
    ui::{
        EditorState,
        components::icon_button::small_icon_button,
        editor::{actions::EditorAction, views::CodeEditorView},
    },
};
use eframe::egui::{self, include_image};
use uuid::Uuid;

impl CodeEditorView {
    pub(crate) fn header(&mut self, ui: &mut egui::Ui, state: &mut EditorState, panel_id: Uuid) {
        let code_buffer = self.code_buffers.entry(panel_id).or_default();

        // Show filename and close button in the header
        if small_icon_button(
            ui,
            egui::Image::new(include_image!("../../../../../assets/icons/reload.svg")),
        )
        .clicked()
        {
            state.actions.push_action(EditorAction::UpdateDirCache);
        }

        let Some(path) = code_buffer.path.as_ref() else {
            return;
        };
        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        if code_buffer.is_modified {
            ui.label(egui::RichText::new(&file_name).strong().bold());
        } else {
            ui.label(egui::RichText::new(&file_name).strong());
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if small_icon_button(
                ui,
                egui::Image::new(include_image!("../../../../../assets/icons/x.svg")),
            )
            .clicked()
            {
                code_buffer.path = None;
            }

            if small_icon_button(
                ui,
                egui::Image::new(include_image!("../../../../../assets/icons/save.svg")),
            )
            .clicked()
            {
                state
                    .actions
                    .push_action(EditorAction::SaveCodeBuffer(panel_id));
            }
        });
    }
}
