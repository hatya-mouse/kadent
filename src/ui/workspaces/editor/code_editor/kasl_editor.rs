use crate::ui::{components::icon_button::small_icon_button, workspaces::EditorUi};
use eframe::egui::{self, TextBuffer, include_image};
use egui_extras::syntax_highlighting::highlight_with;

impl EditorUi {
    pub(super) fn kasl_editor(&mut self, ui: &mut egui::Ui, buffer_index: usize) {
        // Show a placeholder when no file is open
        let Some(Some(_)) = self
            .ui_state
            .code_editor_state
            .code_buffers
            .get(buffer_index)
        else {
            ui.centered_and_justified(|ui| {
                ui.label("Select a file to edit");
            });
            return;
        };

        // Header: filename and close button
        let file_name = self
            .ui_state
            .code_editor_state
            .code_buffers
            .get(buffer_index)
            .and_then(|b| b.as_ref())
            .and_then(|(path, _)| path.file_name())
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();

        let mut close_clicked = false;
        ui.horizontal(|ui| {
            ui.label(&file_name);
            if small_icon_button(
                ui,
                egui::Image::new(include_image!("../../../../../assets/icons/tri_down.svg")),
            )
            .clicked()
            {
                close_clicked = true;
            }
        });

        if close_clicked {
            if let Some(buffer) = self
                .ui_state
                .code_editor_state
                .code_buffers
                .get_mut(buffer_index)
            {
                *buffer = None;
            }
            return;
        }

        // Get a mutable reference to the code buffer at the specified index
        let Some(Some((_, code))) = self
            .ui_state
            .code_editor_state
            .code_buffers
            .get_mut(buffer_index)
        else {
            return;
        };

        // Get the theme and syntect settings for syntax highlighting
        let (Some(theme), Some(syntect_settings)) = (
            self.ui_state.code_editor_state.theme.as_ref(),
            self.ui_state.code_editor_state.syntect_settings.as_ref(),
        ) else {
            return;
        };

        // Create a layouter closure that highlights the code using KASL syntax set
        let mut layouter = |ui: &egui::Ui, buffer: &dyn TextBuffer, wrap_width: f32| {
            let mut layout_job = highlight_with(
                ui.ctx(),
                ui.style(),
                theme,
                buffer.as_str(),
                "kasl",
                syntect_settings,
            );
            layout_job.wrap.max_width = wrap_width;
            ui.fonts_mut(|fonts| fonts.layout_job(layout_job))
        };

        ui.add(
            egui::TextEdit::multiline(code)
                .code_editor()
                .desired_rows(20)
                .lock_focus(true)
                .layouter(&mut layouter),
        );
    }
}
