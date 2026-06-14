use crate::{
    core::metadata::TrackType,
    fonts::RichTextExt,
    ui::{
        components::{
            dialog::dialog,
            text_button::{text_button, text_button_enabled},
            text_input::text_input,
        },
        theme,
        workspaces::{EditorUi, editor::state::DialogState},
    },
};
use eframe::egui;

impl EditorUi {
    pub(crate) fn track_dialog(&mut self, ui: &egui::Ui) {
        let DialogState::AddTrack(mut state) =
            std::mem::replace(&mut self.ui_state.dialog_state, DialogState::None)
        else {
            return;
        };

        let mut should_close = false;

        let modal = dialog(ui, "Add Track", |ui| {
            ui.columns(2, |cols| {
                cols[0].label("Track Type");
                for track_type in [TrackType::Audio, TrackType::Note] {
                    let selected = state.selected_track_type == track_type;
                    if cols[0]
                        .selectable_label(selected, track_type.to_string())
                        .clicked()
                    {
                        state.selected_track_type = track_type;
                    }
                }

                cols[1].label("Track Name");
                text_input(&mut cols[1], &mut state.name);

                cols[1].horizontal(|ui| {
                    if text_button(ui, "cancel_track_creation", "Cancel").clicked() {
                        should_close = true;
                    }

                    let can_create = !state.name.trim().is_empty();
                    text_button_enabled(
                        can_create,
                        ui,
                        "create_track",
                        egui::RichText::new("Create Track").bold(),
                    )
                    .clicked()
                    .then(|| {
                        self.add_track(
                            state.selected_track_type,
                            state.name.clone(),
                            theme::default_track_color(),
                        );
                        should_close = true;
                    });
                });
            });
        });

        if should_close || modal.should_close() {
            self.ui_state.dialog_state = DialogState::None;
        } else {
            self.ui_state.dialog_state = DialogState::AddTrack(state);
        }
    }
}
