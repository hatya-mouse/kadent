mod midi_thread;

use kadent_engine::data_types::MidiEvent;
use midir::MidiInputPort;
use std::{sync::mpsc, thread};

#[derive(Clone)]
pub enum MidiCommand {
    SetMidiPort(MidiInputPort),
    DisconnectMidiPort,
    SendEvent(MidiEvent),
}

pub(crate) fn spawn_midi_thread(
    midi_producer: ringbuf::HeapProd<MidiEvent>,
) -> mpsc::Sender<MidiCommand> {
    // --- MIDI THREAD ---
    let (midi_command_tx, midi_command_rx) = mpsc::channel();
    thread::spawn(move || midi_thread::midi_thread(midi_command_rx, midi_producer));
    midi_command_tx
}
