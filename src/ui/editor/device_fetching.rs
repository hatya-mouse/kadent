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
        self.ui_state.midi_in = Some(midi_in);
        self.ui_state.midi_in_ports = in_ports;

        // Fetch the default output device
        self.ui_state.default_output_device = self.ui_state.host.default_output_device();

        // Fetch the output devices
        self.ui_state.output_devices = self
            .ui_state
            .host
            .output_devices()
            .map(|devices| devices.collect())
            .unwrap_or_default();
    }
}
