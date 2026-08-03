use crate::{
    consts::PANEL_HEADER_MARGIN,
    ui::{
        components::{icon_button::small_icon_button, panel_header::panel_header},
        workspaces::EditorUi,
    },
};
use eframe::egui::{self, TextBuffer, include_image};
use egui_extras::syntax_highlighting::highlight_with;

impl EditorUi {
    pub(super) fn kasl_editor(&mut self, ui: &mut egui::Ui, panel_id: egui::Id) {
        // Show a placeholder when no file is open
        if !self
            .ui_state
            .code_editor_state
            .code_buffers
            .contains_key(&panel_id)
        {
            ui.centered_and_justified(|ui| {
                ui.label("Select a file to edit");
            });
            return;
        };

        // Show filename and close button in the header
        let file_name = self
            .ui_state
            .code_editor_state
            .code_buffers
            .get(&panel_id)
            .and_then(|b| b.as_ref())
            .and_then(|(path, _)| path.file_name())
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();

        let mut close_clicked = false;
        panel_header(ui, PANEL_HEADER_MARGIN, |ui| {
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
                .get_mut(&panel_id)
            {
                *buffer = None;
            }
            return;
        }

        // Get a mutable reference to the code buffer for this panel
        let Some(Some((_, code))) = self
            .ui_state
            .code_editor_state
            .code_buffers
            .get_mut(&panel_id)
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

        // Create a layouter closure that highlights the code using KASL syntax set.
        // wrap_width is forced to infinity to disable line wrapping (horizontal scroll instead).
        let mut layouter = |ui: &egui::Ui, buffer: &dyn TextBuffer, _wrap_width: f32| {
            let mut layout_job = highlight_with(
                ui.ctx(),
                ui.style(),
                theme,
                buffer.as_str(),
                "kasl",
                syntect_settings,
            );
            layout_job.wrap.max_width = f32::INFINITY;
            ui.fonts_mut(|fonts| fonts.layout_job(layout_job))
        };

        // Compute the minimum rows needed to fill the available vertical space
        let row_height = ui.text_style_height(&egui::TextStyle::Monospace);
        let min_rows = (ui.available_height() / row_height).ceil() as usize;
        let desired_rows = code.lines().count().max(min_rows);

        egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(code)
                    .code_editor()
                    .desired_width(f32::INFINITY)
                    .desired_rows(desired_rows)
                    .lock_focus(true)
                    .layouter(&mut layouter),
            );
        });
    }
}
