use crate::{
    actions::EditorAction,
    ui::{
        components::{
            color_picker::color_picker, text_button::text_button, text_input::text_input,
        },
        theme,
        {
            EditorState,
            editor::inspector::{inspector_item, inspector_section},
        },
    },
};
use eframe::egui;
use kadent_engine::mixer::TrackID;

pub(super) fn track_inspector(ui: &mut egui::Ui, state: &mut EditorState, track_id: &TrackID) {
    inspector_section(ui, ("track_section", track_id), "Track", |ui| {
        let Some(track_meta) = state.ui_state.proj_ctx.project_meta.get_track_mut(track_id) else {
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
                state.push_action(EditorAction::RemoveTrack(*track_id));
                state.push_action(EditorAction::DisarmTrack);
            }
        });

        if state.debug_mode {
            ui.separator();
            inspector_item(ui, "Track ID", |ui| {
                ui.label(
                    egui::RichText::new(format!("{}", track_id.0)).size(theme::normal_font_size()),
                );
            });
        }
    });
}
