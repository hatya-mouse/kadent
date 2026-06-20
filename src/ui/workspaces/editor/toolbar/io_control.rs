use crate::{
    core::{metadata::TrackType, midi_input::MidiCommand},
    ui::workspaces::EditorUi,
};
use eframe::egui;
use kadent_engine::thread::AudioCommand;

impl EditorUi {
    pub(super) fn io_control(&mut self, ui: &mut egui::Ui) {
        let label = self
            .ui_state
            .selected_midi_port
            .as_deref()
            .unwrap_or("No MIDI input");

        let Some(midi_in) = &self.ui_state.midi_in else {
            return;
        };
        let midi_in_ports = &self.ui_state.midi_in_ports;
        let midi_in_port_names: Vec<String> = midi_in_ports
            .iter()
            .filter_map(|p| midi_in.port_name(p).ok())
            .collect();

        egui::ComboBox::from_id_salt("midi_port_selector")
            .selected_text(label)
            .width(180.0)
            .show_ui(ui, |ui| {
                if ui
                    .selectable_label(self.ui_state.selected_midi_port.is_none(), "No MIDI input")
                    .clicked()
                    && self.ui_state.selected_midi_port.is_some()
                {
                    self.ui_state.selected_midi_port = None;
                    let _ = self.midi_command_tx.send(MidiCommand::DisconnectMidiPort);
                    let _ = self
                        .thread_handle
                        .audio_command_tx
                        .send(AudioCommand::DisarmTrack);
                }

                for (i, name) in midi_in_port_names.iter().enumerate() {
                    let is_selected =
                        self.ui_state.selected_midi_port.as_deref() == Some(name.as_str());
                    if ui.selectable_label(is_selected, name).clicked()
                        && !is_selected
                        && let Some(port) = midi_in_ports.get(i)
                    {
                        let _ = self
                            .midi_command_tx
                            .send(MidiCommand::SetMidiPort(port.clone()));
                        self.ui_state.selected_midi_port = Some(name.clone());

                        if let Some(&track_id) =
                            self.proj_ctx.project_meta.track_order.iter().find(|id| {
                                self.proj_ctx
                                    .project_meta
                                    .tracks
                                    .get(id)
                                    .is_some_and(|t| t.track_type == TrackType::Note)
                            })
                        {
                            let _ = self
                                .thread_handle
                                .audio_command_tx
                                .send(AudioCommand::ArmTrack(track_id));
                        }
                    }
                }
            });
    }
}
