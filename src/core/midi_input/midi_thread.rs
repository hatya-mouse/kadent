use crate::core::midi_input::MidiCommand;
use kadent_engine::data_types::MidiEvent;
use ringbuf::traits::Producer;
use std::sync::{Arc, Mutex, mpsc};

pub(super) fn midi_thread(
    command_rx: mpsc::Receiver<MidiCommand>,
    midi_producer: ringbuf::HeapProd<MidiEvent>,
) {
    let prod = Arc::new(Mutex::new(midi_producer));
    let mut connection: Option<midir::MidiInputConnection<()>> = None;

    for command in command_rx {
        match command {
            MidiCommand::SetMidiPort(port) => {
                connection.take();

                let Ok(midi_in) = midir::MidiInput::new("kadent_engine") else {
                    eprintln!("Failed to initialize MIDI input");
                    continue;
                };

                let prod_clone = Arc::clone(&prod);
                match midi_in.connect(
                    &port,
                    "kadent_input",
                    move |_, message, _| {
                        push_midi_event(message, &prod_clone);
                    },
                    (),
                ) {
                    Ok(conn) => connection = Some(conn),
                    Err(e) => eprintln!("Failed to connect to MIDI port: {:?}", e.kind()),
                }
            }
            MidiCommand::DisconnectMidiPort => {
                connection.take();
            }
            MidiCommand::SendEvent(event) => {
                if let Ok(mut prod) = prod.lock() {
                    prod.try_push(event).ok();
                }
            }
        }
    }
}

fn push_midi_event(message: &[u8], producer: &Arc<Mutex<ringbuf::HeapProd<MidiEvent>>>) {
    if message.len() < 2 {
        return;
    }
    let status = message[0] & 0xF0;
    let pitch = message[1];
    let velocity = message.get(2).copied().unwrap_or(0);

    // Treat the events with zero velocity as NoteOff
    let event = match (status, velocity) {
        (0x90, velocity) if velocity > 0 => MidiEvent::NoteOn { pitch, velocity },
        (0x90, _) | (0x80, _) => MidiEvent::NoteOff { pitch },
        _ => return,
    };

    if let Ok(mut prod) = producer.try_lock() {
        prod.try_push(event).ok();
    }
}
