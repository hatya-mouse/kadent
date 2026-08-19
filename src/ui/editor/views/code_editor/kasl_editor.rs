use crate::{
    consts::PANEL_HEADER_MARGIN,
    ui::{
        EditorState,
        components::{icon_button::small_icon_button, panel_header::panel_header},
        editor::{PanelView, views::PanelViewState},
    },
};
use eframe::egui::{self, TextBuffer, include_image};
use egui_extras::syntax_highlighting::{CodeTheme, highlight_with};
use uuid::Uuid;

impl EditorState {
    pub(super) fn kasl_editor(&mut self, ui: &mut egui::Ui, panel_id: Uuid) {
        // Get the theme and syntect settings for syntax highlighting
        let theme = CodeTheme::from_memory(ui.ctx(), ui.style());
        let Some(syntect_settings) = self.views.code_editor.syntect_settings.clone() else {
            return;
        };
        // Then get the panel state
        let PanelViewState::CodeEditor(code_buffer) =
            self.views
                .get_panel_state_or_insert(panel_id, PanelView::CodeEditor, || {
                    PanelViewState::CodeEditor(None)
                })
        else {
            return;
        };

        // Show a placeholder when no file is open
        let Some(code_buffer) = code_buffer else {
            ui.centered_and_justified(|ui| {
                ui.label("Select a file to edit");
            });
            return;
        };

        // Show filename and close button in the header
        let file_name = code_buffer
            .0
            .file_name()
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
            self.views
                .insert_panel_state(panel_id, PanelViewState::CodeEditor(None));
            return;
        }

        // Create a layouter closure that highlights the code using KASL syntax set.
        // wrap_width is forced to infinity to disable line wrapping (horizontal scroll instead).
        let mut layouter = |ui: &egui::Ui, buffer: &dyn TextBuffer, _wrap_width: f32| {
            let mut layout_job = highlight_with(
                ui.ctx(),
                ui.style(),
                &theme,
                buffer.as_str(),
                "kasl",
                &syntect_settings,
            );
            layout_job.wrap.max_width = f32::INFINITY;
            ui.fonts_mut(|fonts| fonts.layout_job(layout_job))
        };

        // Compute the minimum rows needed to fill the available vertical space
        let row_height = ui.text_style_height(&egui::TextStyle::Monospace);
        let min_rows = (ui.available_height() / row_height).ceil() as usize;
        let desired_rows = code_buffer.1.lines().count().max(min_rows);

        egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut code_buffer.1)
                    .code_editor()
                    .desired_width(f32::INFINITY)
                    .desired_rows(desired_rows)
                    .lock_focus(true)
                    .layouter(&mut layouter),
            );
        });
    }
}
