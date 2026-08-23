mod note;
mod note_modifier;
mod note_region;
mod processed_note;
mod track_impl;
mod voice_event;

pub(crate) use note::{Note, NoteID};
pub(crate) use note_modifier::{NoteModifier, NoteModifierID};
pub(crate) use note_region::NoteRegion;

use crate::core::audio_engine::{
    data_types::{Event, EventSlot, MidiEvent},
    graph::Graph,
    node::builtin::{AudioOutputNode, NoteInputNode},
    track::{RegionID, note_track::processed_note::ProcessedNote},
};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, VecDeque},
};
use voice_event::VoiceEvent;

#[derive(Default, Clone)]
pub(crate) struct NoteTrack {
    // --- GRAPH ---
    graph: Graph,

    // --- MODIFIERS ---
    modifiers: HashMap<NoteModifierID, Box<dyn NoteModifier>>,

    // --- NOTE DATA ---
    /// The original note data, which is not affected by the modifiers.
    regions: HashMap<RegionID, NoteRegion>,
    /// The processed note data, which has been processed by the modifiers.
    /// This is sorted by the start time of the notes, and is used for generating voice events.
    processed_notes: Vec<ProcessedNote>,

    // --- VOICE EVENTS ---
    /// Voice Events such as NoteOn and NoteOff.
    voice_events: BinaryHeap<Reverse<VoiceEvent>>,
    /// Voice events that could not be processed in the previous sample and are pending to be processed in the next sample.
    delayed_events: VecDeque<Event>,
    /// Pitch values of currently playing sequenced voices, which are used to stop the notes when pausing.
    playing_sequenced_voices: Vec<f32>,

    // --- MIDI VOICE INSERTION ---
    /// Pending MIDI events to be processied in the next buffer.
    pending_midi_events: Vec<MidiEvent>,

    // --- LOCAL BUFFER ---
    /// Local event buffer to store the event slots to be passed to the graph.
    event_buffer: Vec<EventSlot>,
    /// Local buffer to store the calculated audio sample data for this track.
    local_buffer: Vec<f32>,

    // --- MISC ---
    next_region_id: u64,
}

impl NoteTrack {
    pub(crate) fn new() -> Self {
        // Create a graph with the input and output nodes
        let input_node = NoteInputNode::default();
        let output_node = AudioOutputNode::default();
        let graph = Graph::new(Box::new(input_node), Box::new(output_node));

        Self {
            graph,
            ..Default::default()
        }
    }

    pub(crate) fn with_initial(
        graph: Graph,
        regions: HashMap<RegionID, NoteRegion>,
        next_region_id: u64,
    ) -> Self {
        Self {
            graph,
            regions,
            next_region_id,
            ..Default::default()
        }
    }

    // --- REGION GETTING ---

    pub(crate) fn get_region(&self, id: &RegionID) -> Option<&NoteRegion> {
        self.regions.get(id)
    }

    pub(crate) fn get_region_mut(&mut self, id: &RegionID) -> Option<&mut NoteRegion> {
        self.regions.get_mut(id)
    }

    pub(crate) fn get_all_regions(&self) -> &HashMap<RegionID, NoteRegion> {
        &self.regions
    }

    pub(crate) fn take_region(&mut self, id: &RegionID) -> Option<NoteRegion> {
        self.regions.remove(id)
    }

    // --- REGION ADDITION ---

    pub(crate) fn set_next_region_id(&mut self, next_id: u64) {
        self.next_region_id = next_id;
    }

    fn generate_region_id(&mut self) -> RegionID {
        let id = RegionID(self.next_region_id);
        self.next_region_id += 1;
        id
    }

    pub(crate) fn add_region(&mut self, region: NoteRegion) -> RegionID {
        let id = self.generate_region_id();
        self.regions.insert(id, region);
        id
    }

    pub(crate) fn set_regions(&mut self, regions: HashMap<RegionID, NoteRegion>) {
        self.regions = regions;
    }

    // --- REALTIME MIDI ---

    /// Receives live MIDI events and updates the voice state.
    /// Must be called before process() so that changes take effect from sample 0 of the buffer.
    pub(crate) fn pass_midi(&mut self, events: &[MidiEvent]) {
        // Push a new voice event to the queue
        self.pending_midi_events.extend(events.to_vec());
    }
}
