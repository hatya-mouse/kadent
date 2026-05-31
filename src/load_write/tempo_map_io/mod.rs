mod tempo_event;

use crate::load_write::{AsBytes, FromBytes, safe_read};
use kreniq_engine::mixer::{TempoEvent, TempoMap};
use std::io::{Cursor, Read};

impl AsBytes for TempoMap {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write each tempo events
        let mut events_bytes = Vec::new();
        for event in &self.events {
            event.as_bytes(&mut events_bytes);
        }

        // Get the length of the events bytes and write it to the buffer
        bytes.extend(&(events_bytes.len() as u64).to_le_bytes());
        bytes.extend(events_bytes);
    }
}

impl FromBytes for TempoMap {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(bytes);

        // Read the length of the events data
        let mut events_len_bytes = [0u8; 8];
        cursor.read_exact(&mut events_len_bytes)?;
        let events_len = u64::from_le_bytes(events_len_bytes) as usize;

        // Get the events data bytes
        let events_data_bytes = safe_read(&mut cursor, events_len)?;

        // Parse the events from the data bytes
        let mut events = Vec::new();
        let mut events_cursor = Cursor::new(events_data_bytes);
        while events_cursor.position() < events_cursor.get_ref().len() as u64 {
            // Get the bytes for the next event
            let mut event_bytes = [0u8; 24];
            events_cursor.read_exact(&mut event_bytes)?;
            // Parse the event from the bytes
            let event = TempoEvent::from_bytes(&event_bytes)?;
            events.push(event);
        }

        // Construct the new tempo map
        let mut tempo_map = TempoMap::default();
        tempo_map.events = events;

        Ok(tempo_map)
    }
}
