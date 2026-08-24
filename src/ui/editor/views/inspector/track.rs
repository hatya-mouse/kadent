use super::{inspector_item, inspector_section};
use crate::core::audio_engine::mixer::TrackID;
use crate::ui::editor::views::inspector::InspectorView;
use crate::{
    ui::editor::actions::EditorAction,
    ui::{
        EditorState,
        components::{
            color_picker::color_picker, text_button::text_button, text_input::text_input,
        },
        theme,
    },
};
use eframe::egui;

impl InspectorView {
    pub(super) fn track_inspector(
        &mut self,
        ui: &mut egui::Ui,
        editor_state: &mut EditorState,
        track_id: &TrackID,
    ) {
        inspector_section(ui, ("track_section", track_id), "Track", |ui| {
            let Some(track_meta) = editor_state.project.meta.get_track_mut(track_id) else {
                return;
            };

            inspector_item(ui, "Name", |ui| {
                text_input(ui, &mut track_meta.name);
            });

            inspector_item(ui, "Color", |ui| {
                color_picker(ui, &mut track_meta.color);
            });

            inspector_item(ui, "Delete", |ui| {
                if text_button(ui, "delete_track", "Delete Track").clicked() {
                    editor_state
                        .actions
                        .push_action(EditorAction::RemoveTrack(*track_id));
                    editor_state.actions.push_action(EditorAction::DisarmTrack);
                }
            });

            if editor_state.debug_mode {
                ui.separator();
                inspector_item(ui, "Track ID", |ui| {
                    ui.label(
                        egui::RichText::new(format!("{}", track_id.0))
                            .size(theme::normal_font_size()),
                    );
                });
            }
        });
    }
}
