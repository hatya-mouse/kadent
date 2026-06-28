use crate::ui::workspaces::EditorUi;
use eframe::egui::{self, TextBuffer};
use egui_extras::syntax_highlighting::highlight_with;

impl EditorUi {
    pub(super) fn kasl_editor(&mut self, ui: &mut egui::Ui, buffer_index: usize) {
        // Get a mutable reference to the code buffer at the specified index
        let Some(code) = self
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
