use crate::ui::{
    components::not_available_text::not_available_text, editor::views::code_editor::CodeEditorView,
    theme,
};
use eframe::egui::{self, TextBuffer};
use egui_extras::syntax_highlighting::{CodeTheme, highlight_with};
use kasl::core::error::ErrorRecord;
use std::time::Instant;
use uuid::Uuid;

const LINE_NUMBER_WIDTH: f32 = 40.0;
const LINE_NUMBER_MARGIN: egui::Vec2 = egui::vec2(6.0, 4.0);
const CODE_EDITOR_MARGIN: egui::Margin = egui::Margin::symmetric(6, 4);
const CODE_EDITOR_FONT_SIZE: f32 = 14.0;

impl CodeEditorView {
    pub(super) fn kasl_editor(&mut self, ui: &mut egui::Ui, panel_id: Uuid) {
        let code_buffer = self.code_buffers.entry(panel_id).or_default();
        if code_buffer.path.is_none() {
            not_available_text(ui, "Select a File to Edit");
            return;
        }

        // Get the theme and syntect settings for syntax highlighting
        let theme = CodeTheme::from_memory(ui.ctx(), ui.style());
        let Some(syntect_settings) = self.syntect_settings.clone() else {
            return;
        };

        // Create a layouter closure that highlights the code using KASL syntax set.
        // wrap_width is forced to infinity to disable line wrapping (horizontal scroll instead).
        let editor_font = egui::FontId::monospace(CODE_EDITOR_FONT_SIZE);
        let mut layouter = |ui: &egui::Ui, buffer: &dyn TextBuffer, _wrap_width: f32| {
            let mut layout_job = highlight_with(
                ui.ctx(),
                ui.style(),
                &theme,
                buffer.as_str(),
                "kasl",
                &syntect_settings,
            );

            for section in &mut layout_job.sections {
                section.format.font_id = editor_font.clone();
            }

            // Highlight the errors
            highlight_errors(
                &mut layout_job,
                &code_buffer.errors,
                code_buffer.has_modified_since_last_lint,
            );

            layout_job.wrap.max_width = f32::INFINITY;
            ui.fonts_mut(|fonts| fonts.layout_job(layout_job))
        };

        // Calculate the minimum rows needed to fill the available vertical space
        let buffer_rows =
            code_buffer.content.lines().count() + usize::from(code_buffer.content.ends_with('\n'));
        let editor_font = egui::FontId::monospace(CODE_EDITOR_FONT_SIZE);
        let font_row_height = ui.fonts_mut(|fonts| fonts.row_height(&editor_font));
        let min_rows = (ui.available_height() / font_row_height).ceil() as usize;
        let desired_rows = buffer_rows.max(min_rows);

        let scroll_response = egui::ScrollArea::both()
            .id_salt("kasl_editor")
            .scroll_offset(code_buffer.scroll_offset)
            .auto_shrink(false)
            .show(ui, |ui| {
                ui.allocate_ui_with_layout(
                    ui.available_size(),
                    egui::Layout::left_to_right(egui::Align::TOP),
                    |ui| {
                        ui.add_space(LINE_NUMBER_WIDTH);
                        egui::TextEdit::multiline(&mut code_buffer.content)
                            .frame(egui::Frame::new().inner_margin(CODE_EDITOR_MARGIN))
                            .id_salt(&code_buffer.path)
                            .desired_width(f32::INFINITY)
                            .desired_rows(desired_rows)
                            .lock_focus(true)
                            .layouter(&mut layouter)
                            .show(ui)
                    },
                )
            });
        code_buffer.scroll_offset = scroll_response.state.offset;

        // Check if the buffer has been modified
        let output = scroll_response.inner.inner;
        if output.response.changed() {
            code_buffer.is_modified = true;
            code_buffer.has_modified_since_last_lint = true;
            code_buffer.last_edit_time = Some(Instant::now())
        }

        // Render the line numbers
        let painter = ui.painter_at(scroll_response.inner_rect);
        for (line_number, row) in output.galley.rows.iter().enumerate() {
            let row_pos =
                scroll_response.inner_rect.min + LINE_NUMBER_MARGIN + egui::vec2(0.0, row.max_y())
                    - code_buffer.scroll_offset;
            painter.text(
                row_pos,
                egui::Align2::LEFT_BOTTOM,
                line_number + 1,
                editor_font.clone(),
                theme::secondary_fg(ui.visuals().dark_mode),
            );
        }
    }
}

fn highlight_errors(
    layout_job: &mut egui::text::LayoutJob,
    errors: &[ErrorRecord],
    has_modified_since_last_lint: bool,
) {
    let error_color = if has_modified_since_last_lint {
        egui::Color32::GRAY
    } else {
        theme::error_fg()
    };

    for error in errors.iter() {
        for range in &error.ranges {
            let start = range.start;
            let end = range.end;

            for section in &mut layout_job.sections {
                let section_start = section.byte_range.start.0;
                let section_end = section.byte_range.end.0;

                if start < section_end && end > section_start {
                    section.format.underline = egui::Stroke::new(1.5, error_color);
                }
            }
        }
    }
}
