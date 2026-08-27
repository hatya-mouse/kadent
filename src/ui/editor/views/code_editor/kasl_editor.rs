use crate::ui::{
    components::not_available_text::not_available_text,
    editor::views::{
        CodeEditorPanelState,
        code_editor::{BufferErrorState, CODE_EDITOR_FONT_SIZE, CodeEditorView},
    },
    theme,
};
use eframe::egui::{
    self, TextBuffer,
    text::{CCursor, CCursorRange},
};
use egui_extras::syntax_highlighting::{CodeTheme, highlight_with};
use kasl::core::error::Severity;
use std::time::Instant;
use uuid::Uuid;

const LINE_NUMBER_WIDTH: f32 = 40.0;
const LINE_NUMBER_MARGIN: egui::Vec2 = egui::vec2(6.0, 4.0);
const CODE_EDITOR_MARGIN: egui::Margin = egui::Margin::symmetric(6, 4);

const COMPILER_BUG_COLOR: egui::Color32 = egui::Color32::from_rgb(185, 101, 218);
const ERROR_COLOR: egui::Color32 = egui::Color32::from_rgb(245, 34, 56);
const WARNING_COLOR: egui::Color32 = egui::Color32::from_rgb(222, 164, 54);

impl CodeEditorView {
    pub(super) fn kasl_editor(
        &mut self,
        ui: &mut egui::Ui,
        panel_id: Uuid,
        panel_state: &mut CodeEditorPanelState,
    ) {
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
            highlight_errors(&mut layout_job, &code_buffer.errors);

            layout_job.wrap.max_width = f32::INFINITY;
            ui.fonts_mut(|fonts| fonts.layout_job(layout_job))
        };

        // Calculate the minimum rows needed to fill the available vertical space
        let buffer_rows =
            code_buffer.content.lines().count() + usize::from(code_buffer.content.ends_with('\n'));
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
            code_buffer.errors.has_modified_since_last_lint = true;
            code_buffer.errors.last_edit_time = Some(Instant::now())
        }

        if let Some((start, end)) = panel_state.jump_index.take() {
            output.response.request_focus();
            // Create a cursor to jump to the clicked error
            let new_range = CCursorRange::two(CCursor::new(start), CCursor::new(end));
            // Set the cursor to the new range
            let mut state = output.state;
            state.cursor.set_char_range(Some(new_range));
            state.store(ui.ctx(), output.response.id);
        }

        // Render the line numbers
        let painter = ui.painter_at(scroll_response.inner_rect);
        for (line_number, row) in output.galley.rows.iter().enumerate() {
            if row.max_y() < code_buffer.scroll_offset.y
                || row.min_y() > code_buffer.scroll_offset.y + scroll_response.inner_rect.height()
            {
                continue;
            }

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

fn highlight_errors(layout_job: &mut egui::text::LayoutJob, errors: &BufferErrorState) {
    let BufferErrorState {
        has_modified_since_last_lint,
        records,
        byte_offsets,
        ..
    } = errors;

    for record in records.iter() {
        let error_color = if *has_modified_since_last_lint {
            egui::Color32::GRAY
        } else {
            match record.severity {
                Severity::CompilerBug => COMPILER_BUG_COLOR,
                Severity::Error => ERROR_COLOR,
                Severity::Warning => WARNING_COLOR,
            }
        };

        for range in &record.ranges {
            let start = range.start;
            let mut end = range.end;
            if start == end {
                end += 1;
            }
            let start_bytes = byte_offsets.get(start).copied().unwrap_or_default();
            let end_bytes = byte_offsets.get(end).copied().unwrap_or_default();

            for section in &mut layout_job.sections {
                let section_start = section.byte_range.start.0;
                let section_end = section.byte_range.end.0;

                if start_bytes < section_end && end_bytes > section_start {
                    section.format.underline = egui::Stroke::new(1.5, error_color);
                }
            }
        }
    }
}
