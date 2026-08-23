use midir::{MidiInput, MidiInputPorts};

#[derive(Default)]
pub(crate) struct MidiDeviceManager {
    /// The name of the currently connected MIDI input port.
    pub(crate) selected_port: Option<String>,
    /// The MIDI input that communicates with the selected port.
    pub(crate) input: Option<MidiInput>,
    /// The names of the available MIDI input ports.
    pub(crate) in_ports: MidiInputPorts,
}
