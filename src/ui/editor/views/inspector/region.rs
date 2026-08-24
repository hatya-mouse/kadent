use super::{inspector_item, inspector_section};
use crate::core::audio_engine::{mixer::TrackID, track::RegionID};
use crate::ui::editor::views::inspector::InspectorView;
use crate::ui::{EditorState, components::text_input::text_input, theme};
use eframe::egui;

impl InspectorView {
    pub(super) fn region_inspector(
        &mut self,
        ui: &mut egui::Ui,
        editor_state: &mut EditorState,
        track_id: &TrackID,
        region_id: &RegionID,
    ) {
        let Some(track_meta) = editor_state.project.meta.get_track_mut(track_id) else {
            return;
        };
        let Some(region_meta) = track_meta.get_region_mut(region_id) else {
            return;
        };

        inspector_section(
            ui,
            ("region_section", track_id, region_id),
            "Region",
            |ui| {
                inspector_item(ui, "Name", |ui| {
                    text_input(ui, &mut region_meta.name);
                });

                if editor_state.debug_mode {
                    ui.separator();
                    inspector_item(ui, "Region ID", |ui| {
                        ui.label(
                            egui::RichText::new(format!("{}", region_id.0))
                                .size(theme::normal_font_size()),
                        );
                    });
                }
            },
        );
    }
}
