use midir::{MidiInput, MidiInputPorts};

#[derive(Default)]
pub(crate) struct MidiDeviceManager {
    /// The name of the currently connected MIDI input port.
    pub selected_port: Option<String>,
    /// The MIDI input that communicates with the selected port.
    pub input: Option<MidiInput>,
    /// The names of the available MIDI input ports.
    pub in_ports: MidiInputPorts,
}
