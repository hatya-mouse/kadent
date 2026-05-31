use crate::{ui::EditorUi, ui_state::dialog_state::TrackType};
use eframe::egui;
use kreniq_engine::thread::{AudioCommand, MidiCommand};
use midir::MidiInput;

impl EditorUi {
    pub(super) fn midi_control(&mut self, ui: &mut egui::Ui) {
        let Ok(midi_in) = MidiInput::new("kreniq") else {
            return;
        };

        let ports = midi_in.ports();
        let port_names: Vec<String> = ports
            .iter()
            .filter_map(|p| midi_in.port_name(p).ok())
            .collect();

        let label = self
            .selected_midi_port
            .as_deref()
            .unwrap_or("No MIDI input");

        egui::ComboBox::from_id_salt("midi_port_selector")
            .selected_text(label)
            .width(180.0)
            .show_ui(ui, |ui| {
                if ui
                    .selectable_label(self.selected_midi_port.is_none(), "No MIDI input")
                    .clicked()
                    && self.selected_midi_port.is_some()
                {
                    self.selected_midi_port = None;
                    let _ = self
                        .thread_handle
                        .midi_command_tx
                        .send(MidiCommand::DisconnectMidiPort);
                    let _ = self
                        .thread_handle
                        .audio_command_tx
                        .send(AudioCommand::DisarmTrack);
                }

                for (i, name) in port_names.iter().enumerate() {
                    let is_selected = self.selected_midi_port.as_deref() == Some(name.as_str());
                    if ui.selectable_label(is_selected, name).clicked()
                        && !is_selected
                        && let Some(port) = ports.get(i)
                    {
                        let _ = self
                            .thread_handle
                            .midi_command_tx
                            .send(MidiCommand::SetMidiPort(port.clone()));
                        self.selected_midi_port = Some(name.clone());

                        if let Some(&track_id) = self.project_meta.track_order.iter().find(|id| {
                            self.project_meta
                                .tracks
                                .get(id)
                                .is_some_and(|t| t.track_type == TrackType::Note)
                        }) {
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
