use crate::{
    components::{dialog::dialog, text_input::text_input},
    theme,
    ui::EditorUi,
    ui_state::dialog_state::{DialogState, TrackType},
};
use eframe::egui;

impl EditorUi {
    pub(crate) fn track_dialog(&mut self, ui: &egui::Ui) {
        let DialogState::AddTrack(mut state) =
            std::mem::replace(&mut self.ui_state.dialog_state, DialogState::None)
        else {
            return;
        };

        let mut close = false;

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

                let name_empty = state.name.trim().is_empty();
                cols[1]
                    .add_enabled(!name_empty, egui::Button::new("Create"))
                    .clicked()
                    .then(|| {
                        self.add_track(
                            state.selected_track_type,
                            state.name.clone(),
                            theme::default_track_color(),
                        );
                        close = true;
                    });
            });

            if ui.button("Cancel").clicked() {
                close = true;
            }
        });

        if close || modal.should_close() {
            self.ui_state.dialog_state = DialogState::None;
        } else {
            self.ui_state.dialog_state = DialogState::AddTrack(state);
        }
    }
}
