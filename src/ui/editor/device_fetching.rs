use crate::ui::EditorState;
use cpal::traits::HostTrait;
use midir::MidiInput;

impl EditorState {
    /// Fetches the audio devices from the host.
    pub(super) fn fetch_devices(&mut self) {
        // Fetch the available MIDI ports
        let Ok(midi_in) = MidiInput::new("kadent") else {
            return;
        };

        let in_ports = midi_in.ports();
        self.midi_device.input = Some(midi_in);
        self.midi_device.in_ports = in_ports;

        // Fetch the default output device
        self.audio_device.default_output = self.audio_device.host.default_output_device();

        // Fetch the output devices
        self.audio_device.outputs = self
            .audio_device
            .host
            .output_devices()
            .map(|devices| devices.collect())
            .unwrap_or_default();
    }
}
