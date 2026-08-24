use super::{inspector_item, inspector_section};
use crate::ui::{EditorState, theme};
use crate::{
    core::audio_engine::{
        mixer::TrackID,
        track::{RegionID, note_track::NoteID},
    },
    ui::editor::views::inspector::InspectorView,
};
use eframe::egui;

impl InspectorView {
    pub(super) fn note_inspector(
        &mut self,
        ui: &mut egui::Ui,
        editor_state: &mut EditorState,
        track_id: &TrackID,
        region_id: &RegionID,
        note_id: &NoteID,
    ) {
        inspector_section(
            ui,
            ("note_section", track_id, region_id, note_id),
            "Note",
            |ui| {
                if editor_state.debug_mode {
                    ui.separator();
                    inspector_item(ui, "Note ID", |ui| {
                        ui.label(
                            egui::RichText::new(format!("{}", note_id.0))
                                .size(theme::normal_font_size()),
                        );
                    });
                }
            },
        );
    }
}
