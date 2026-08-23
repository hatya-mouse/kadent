use crate::core::audio_engine::{mixer::TrackID, thread::AudioCommand};
use crate::{core::midi_thread::MidiCommand, ui::EditorState};
use midir::MidiInputPort;

impl EditorState {
    pub(super) fn set_midi_input_port(&mut self, midi_in_port: MidiInputPort) {
        self.midi_device.selected_port = Some(midi_in_port.id());
        self.midi_tx
            .send(MidiCommand::SetMidiPort(midi_in_port))
            .ok();
    }

    pub(super) fn disconnect_midi_port(&mut self) {
        self.midi_device.selected_port = None;
        self.midi_tx.send(MidiCommand::DisconnectMidiPort).ok();
        self.disarm_track();
    }

    pub(super) fn arm_track(&mut self, track_id: TrackID) {
        self.thread_handle
            .audio_command_tx
            .send(AudioCommand::ArmTrack(track_id))
            .ok();
    }

    pub(super) fn disarm_track(&mut self) {
        self.thread_handle
            .audio_command_tx
            .send(AudioCommand::DisarmTrack)
            .ok();
    }
}
