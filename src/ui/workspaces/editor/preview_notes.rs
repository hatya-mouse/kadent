use crate::{core::midi_thread::MidiCommand, ui::workspaces::EditorUi};
use kadent_engine::data_types::MidiEvent;

const PREVIEW_NOTE_DURATION: std::time::Duration = std::time::Duration::from_millis(500);

impl EditorUi {
    pub(super) fn play_note_feedback(&mut self, pitch: u8, velocity: u8) {
        self.midi_command_tx
            .send(MidiCommand::SendEvent(MidiEvent::NoteOn {
                pitch,
                velocity,
            }))
            .ok();
        // Add the note to the preview notes with the current timestamp
        self.ui_state
            .piano_roll_state
            .preview_notes
            .push((pitch, std::time::Instant::now()));
    }

    pub(super) fn update_preview_notes(&mut self) {
        let now = std::time::Instant::now();
        let tx = &self.midi_command_tx;

        self.ui_state
            .piano_roll_state
            .preview_notes
            .retain(|&(pitch, started_at)| {
                // If the note has been playing for longer than PREVIEW_NOTE_DURATION,
                // send a NoteOff event and remove it from the preview notes
                if now.duration_since(started_at) >= PREVIEW_NOTE_DURATION {
                    tx.send(MidiCommand::SendEvent(MidiEvent::NoteOff { pitch }))
                        .ok();
                    false
                } else {
                    true
                }
            });
    }
}
