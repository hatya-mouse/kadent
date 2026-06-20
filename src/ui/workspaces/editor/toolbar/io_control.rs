use crate::{
    core::{metadata::TrackType, midi_input::MidiCommand},
    ui::workspaces::EditorUi,
};
use cpal::traits::DeviceTrait;
use eframe::egui::{self, include_image};
use kadent_engine::thread::AudioCommand;

impl EditorUi {
    pub(super) fn io_control(&mut self, ui: &mut egui::Ui) {
        let mut audio_output_item =
            |ui: &mut egui::Ui, device_name: String, device: &cpal::Device| {
                let device_id = device.id().ok();
                // Check if the device is currently selected
                let is_selected = self.ui_state.selected_output_device == device_id;

                if ui.selectable_label(is_selected, &device_name).clicked() && !is_selected {
                    // Select the label when clicked
                    self.ui_state.selected_output_device = device_id;
                    self.thread_handle
                        .audio_command_tx
                        .send(AudioCommand::SetOutputDevice(device.clone()))
                        .ok();
                }
            };

        let headphone_img = include_image!("../../../../../assets/icons/tri_down.svg");
        ui.menu_image_button(headphone_img, |ui| {
            ui.set_min_width(180.0);

            // --- AUDIO OUTPUT DEVICE ---
            ui.menu_button("Audio Output", |ui| {
                if let Some(default_output_device) = &self.ui_state.default_output_device {
                    let default_output_name =
                        format!("Default — {}", get_device_name(default_output_device));
                    audio_output_item(ui, default_output_name, default_output_device);
                }

                let output_devices: Vec<(String, &cpal::Device)> = self
                    .ui_state
                    .output_devices
                    .iter()
                    .map(|device| (get_device_name(device), device))
                    .collect();
                for (device_name, device) in output_devices {
                    audio_output_item(ui, device_name, device);
                }
            });

            // --- MIDI INPUT PORTS ---
            ui.menu_button("MIDI Input", |ui| {
                // Get the names of available MIDI input ports
                let Some(midi_in) = &self.ui_state.midi_in else {
                    return;
                };
                let midi_in_ports = &self.ui_state.midi_in_ports;
                let midi_in_port_names: Vec<String> = midi_in_ports
                    .iter()
                    .filter_map(|p| midi_in.port_name(p).ok())
                    .collect();

                if ui
                    .selectable_label(self.ui_state.selected_midi_port.is_none(), "No MIDI input")
                    .clicked()
                    && self.ui_state.selected_midi_port.is_some()
                {
                    self.ui_state.selected_midi_port = None;
                    self.midi_command_tx
                        .send(MidiCommand::DisconnectMidiPort)
                        .ok();
                    self.thread_handle
                        .audio_command_tx
                        .send(AudioCommand::DisarmTrack)
                        .ok();
                }

                for (i, name) in midi_in_port_names.iter().enumerate() {
                    let is_selected =
                        self.ui_state.selected_midi_port.as_deref() == Some(name.as_str());
                    if ui.selectable_label(is_selected, name).clicked()
                        && !is_selected
                        && let Some(port) = midi_in_ports.get(i)
                    {
                        self.midi_command_tx
                            .send(MidiCommand::SetMidiPort(port.clone()))
                            .ok();
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
                            self.thread_handle
                                .audio_command_tx
                                .send(AudioCommand::ArmTrack(track_id))
                                .ok();
                        }
                    }
                }
            });
        });
    }
}

fn get_device_name(device: &cpal::Device) -> String {
    device
        .description()
        .map(|desc| desc.name().to_string())
        .unwrap_or_else(|_| "Unknown Device".to_string())
}
