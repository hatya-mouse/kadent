use crate::core::audio_engine::thread::AudioCommand;
use crate::{
    ui::editor::actions::EditorAction,
    ui::{
        components::toolbar_button::toolbar_icon_button,
        theme,
        {EditorState, editor::toolbar::toolbar_group},
    },
};
use cpal::traits::DeviceTrait;
use eframe::egui::{
    self,
    containers::menu::{self, MenuConfig},
    include_image,
    style::StyleModifier,
};
use midir::MidiInputPort;

/// Displays the IO control menu for selecting audio output devices and MIDI input ports.
pub(super) fn io_control(ui: &mut egui::Ui, state: &mut EditorState) {
    let menu_style = theme::menu_style(ui);

    let headphones_icon = include_image!("../../../../assets/icons/headphones.svg");
    let keyboard_icon = include_image!("../../../../assets/icons/keyboard.svg");

    toolbar_group(ui, |ui| {
        let response = toolbar_icon_button(ui, egui::Image::new(headphones_icon.clone()));

        if response.clicked() {
            egui::Popup::toggle_id(&response.ctx, response.id);
        }

        egui::Popup::menu(&response)
            .style(menu_style.clone())
            .show(|ui| {
                ui.set_min_width(160.0);

                // --- AUDIO OUTPUT DEVICE ---
                let mut audio_output_item =
                    |ui: &mut egui::Ui, device_name: String, device: &cpal::Device| {
                        let device_id = device.id().ok();
                        // Check if the device is currently selected
                        let is_selected = state.audio_device.selected_output == device_id;

                        if ui.selectable_label(is_selected, &device_name).clicked() && !is_selected
                        {
                            state
                                .actions
                                .push_action(EditorAction::SetAudioOutputDevice(device.clone()));
                        }
                    };

                menu::SubMenuButton::from_button(egui::Button::image_and_text(
                    egui::Image::new(headphones_icon)
                        .tint(theme::primary_fg(ui.visuals().dark_mode)),
                    "Audio Output",
                ))
                .config(MenuConfig::new().style(StyleModifier::from(menu_style.clone())))
                .ui(ui, |ui| {
                    if let Some(default_output_device) = &state.audio_device.default_output {
                        let default_output_name =
                            format!("Default — {}", get_device_name(default_output_device));
                        audio_output_item(ui, default_output_name, default_output_device);
                    }

                    let output_devices: Vec<(String, &cpal::Device)> = state
                        .audio_device
                        .outputs
                        .iter()
                        .map(|device| (get_device_name(device), device))
                        .collect();
                    for (device_name, device) in output_devices {
                        audio_output_item(ui, device_name, device);
                    }
                });

                // --- MIDI INPUT PORTS ---
                menu::SubMenuButton::from_button(egui::Button::image_and_text(
                    egui::Image::new(keyboard_icon).tint(theme::primary_fg(ui.visuals().dark_mode)),
                    "MIDI Input",
                ))
                .config(MenuConfig::new().style(StyleModifier::from(menu_style.clone())))
                .ui(ui, |ui| {
                    // Get the names of available MIDI input ports
                    let Some(midi_in) = &state.midi_device.input else {
                        return;
                    };
                    let midi_in_port_names: Vec<(String, MidiInputPort)> = state
                        .midi_device
                        .in_ports
                        .iter()
                        .filter_map(|midi_in_port| {
                            midi_in
                                .port_name(midi_in_port)
                                .ok()
                                .map(|port_name| (port_name, midi_in_port.clone()))
                        })
                        .collect();

                    if ui
                        .selectable_label(
                            state.midi_device.selected_port.is_none(),
                            "No MIDI Input",
                        )
                        .clicked()
                        && state.midi_device.selected_port.is_some()
                    {
                        state.actions.push_action(EditorAction::DisconnectMidiPort);
                    }

                    for (ref name, midi_in_port) in midi_in_port_names {
                        let is_selected =
                            state.midi_device.selected_port.as_ref() == Some(&midi_in_port.id());
                        if ui.selectable_label(is_selected, name).clicked() && !is_selected {
                            // Set the selected MIDI input port
                            state
                                .actions
                                .push_action(EditorAction::SetMidiInputPort(midi_in_port));
                        }
                    }
                });

                // --- FETCH LATEST DEVICES ---
                if ui.button("Refresh Devices").clicked() {
                    state.fetch_devices();
                }
            });
    });
}

/// Helper function to get the name of the CPAL device.
fn get_device_name(device: &cpal::Device) -> String {
    device
        .description()
        .map(|desc| desc.name().to_string())
        .unwrap_or_else(|_| "Unknown Device".to_string())
}
