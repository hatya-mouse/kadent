use crate::core::audio_engine::data_types::MidiEvent;
use crate::core::midi_thread::MidiCommand;
use crate::ui::editor::EditorUi;

const PREVIEW_NOTE_DURATION: std::time::Duration = std::time::Duration::from_millis(500);

impl EditorUi {
    pub(super) fn update_preview_notes(&mut self) {
        let now = std::time::Instant::now();
        let tx = &self.state.midi_tx;

        self.views
            .piano_roll
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
