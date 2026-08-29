use crate::{
    core::metadata::TrackType,
    fonts::RichTextExt,
    ui::{
        EditorState,
        components::{
            dialog::{dialog, dialog_bold_label},
            text_button::{text_button, text_button_enabled},
            text_input::text_input,
        },
        editor::{DialogState, actions::EditorAction},
        theme,
    },
};
use eframe::egui;

impl DialogState {
    pub(super) fn track_dialog(&mut self, ui: &egui::Ui, state: &mut EditorState) {
        let DialogState::AddTrack {
            selected_track_type,
            name,
        } = self
        else {
            return;
        };

        let mut should_close = false;

        let modal = dialog(ui, "Add Track", 6, |ui| {
            ui.columns(2, |cols| {
                dialog_bold_label(&mut cols[0], "Track Type");

                *cols[0].style_mut() = theme::menu_style(&cols[0]);
                TrackType::all().iter().for_each(|track_type| {
                    let selected = selected_track_type == track_type;
                    if cols[0]
                        .selectable_label(selected, track_type.to_string())
                        .clicked()
                    {
                        *selected_track_type = *track_type;
                    }
                });

                dialog_bold_label(&mut cols[1], "Track Name");
                text_input(&mut cols[1], name);

                cols[1].horizontal(|ui| {
                    if text_button(ui, "cancel_track_creation", "Cancel").clicked() {
                        should_close = true;
                    }

                    let can_create = !name.trim().is_empty();
                    text_button_enabled(
                        can_create,
                        true,
                        ui,
                        "create_track",
                        egui::RichText::new("Create Track").bold(),
                    )
                    .clicked()
                    .then(|| {
                        state.actions.push_action(EditorAction::AddTrack(
                            *selected_track_type,
                            name.clone(),
                            theme::default_track_color(),
                        ));
                        should_close = true;
                    });
                });
            });
        });

        if should_close || modal.should_close() {
            *self = DialogState::None;
        }
    }
}
