mod process;

use crate::core::audio_engine::{
    MAX_CHANNELS,
    audio_data::AudioFilePool,
    data_types::{AudioContext, PlaybackContext},
    graph::Graph,
    timing::{TempoMap, TimeBounds},
    track::{
        RegionID, Track,
        error::TrackError,
        note_track::{NoteTrack, VoiceEvent},
    },
};
use std::cmp::Reverse;

impl Track for NoteTrack {
    // --- CLONING ---

    fn clone_box(&self) -> Box<dyn Track> {
        Box::new(self.clone())
    }

    // --- GRAPH GETTING ---

    fn get_graph(&self) -> &Graph {
        &self.graph
    }

    fn get_graph_mut(&mut self) -> &mut Graph {
        &mut self.graph
    }

    // --- REGION BOUNDS GETTING ---

    fn get_region_bounds(&self, region_id: &RegionID) -> Option<&TimeBounds> {
        self.regions.get(region_id).map(|region| &region.bounds)
    }

    // --- REGION MODIFICATION ---

    fn set_region_bounds(&mut self, region_id: &RegionID, new_bounds: TimeBounds) {
        if let Some(region) = self.regions.get_mut(region_id) {
            region.bounds = new_bounds;
        }
    }

    fn remove_region(&mut self, region_id: &RegionID) {
        self.regions.remove(region_id);
    }

    // --- SEEKING ---

    fn seek(&mut self, _playhead: usize, _playback_ctx: &PlaybackContext) {
        // Clear the voices and events
        self.voice_events.clear();
        // Pause and clear the playing sequenced notes
        self.stop_sequenced_notes();
    }

    // --- TRACK PROCESSING ---

    fn prepare(
        &mut self,
        _audio_pool: &mut AudioFilePool,
        tempo_map: &TempoMap,
        _audio_ctx: &AudioContext,
        playback_ctx: &PlaybackContext,
    ) -> Result<(), TrackError> {
        // Pre-process the sequenced notes into processed notes
        self.pre_process_notes(tempo_map);

        // Clear the voices and events
        self.voice_events.clear();
        // Initialize the local buffer
        self.event_buffer = Vec::with_capacity(playback_ctx.buffer_size);
        self.local_buffer = vec![0.0; playback_ctx.buffer_size * MAX_CHANNELS];
        // Prepare the graph
        self.graph
            .prepare(tempo_map, playback_ctx)
            .map_err(TrackError::GraphError)
    }

    fn process_to_local_buffer(
        &mut self,
        is_playing: bool,
        playhead: usize,
        tempo_map: &TempoMap,
        playback_ctx: &PlaybackContext,
    ) {
        self.event_buffer.clear();
        let buffer_end = playhead + playback_ctx.buffer_size;

        // Convert the pending MIDI notes to voice events and push them to the voice_events vector
        let converted_midi_events: Vec<Reverse<VoiceEvent>> = self
            .pending_midi_events
            .drain(..)
            .map(|midi_event| Reverse(VoiceEvent::from_midi_event(playhead, midi_event)))
            .collect();
        self.voice_events.extend(converted_midi_events);

        if is_playing {
            // Create voice events from sequenced notes
            self.create_events_from_notes(playhead, tempo_map, playback_ctx);
        } else {
            // Is it is paused, pause all sequenced notes
            self.stop_sequenced_notes();
        }

        for sample in playhead..buffer_end {
            // Convert voice events to events and store it to the event buffer
            self.consume_events_at_sample(sample);
        }

        // Get a pointer to the voice buffer
        let input_ptr = self.event_buffer.as_ptr() as *const u8;
        // Process the graph
        self.graph.process(
            &[input_ptr],
            &[self.local_buffer.as_mut_ptr() as *mut u8],
            playhead,
            playback_ctx,
        );
    }

    fn get_local_buffer(&self) -> &[f32] {
        &self.local_buffer
    }

    // --- ANY CASTING ---

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
