use crate::{
    core::audio_engine::track::{
        RegionID, Track,
        note_track::{Note, NoteID, NoteRegion, NoteTrack},
    },
    storage::project::serial::restore_next_id,
};
use sode::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder};
use std::collections::HashMap;

// --- NoteTrack ---

impl Encode for NoteTrack {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, self.get_graph())?;
        e.field(1, self.get_all_regions())?;
        Ok(())
    }
}

impl Decode for NoteTrack {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        let graph = d.field(0)?.ok_or(DecodeError::InvalidData)?;
        let regions: HashMap<RegionID, NoteRegion> = d.field(1)?.ok_or(DecodeError::InvalidData)?;
        let next_id = restore_next_id(&regions.keys().copied().collect::<Vec<_>>());
        Ok(NoteTrack::with_initial(graph, regions, next_id))
    }
}

// --- NoteRegion ---

impl Encode for NoteRegion {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, &self.bounds)?;
        e.field(1, &self.notes)?;
        Ok(())
    }
}

impl Decode for NoteRegion {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        let bounds = d.field(0)?.ok_or(DecodeError::InvalidData)?;
        let notes: HashMap<NoteID, Note> = d.field(1)?.ok_or(DecodeError::InvalidData)?;
        let next_id = restore_next_id(&notes.keys().copied().collect::<Vec<_>>());
        Ok(NoteRegion::with_notes(bounds, notes, next_id))
    }
}

// --- Note ---

impl Encode for Note {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, &self.start)?;
        e.field(1, &self.duration)?;
        e.field(2, &self.pitch)?;
        e.field(3, &self.velocity)?;
        Ok(())
    }
}

impl Decode for Note {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        Ok(Note::new(
            d.field(0)?.ok_or(DecodeError::InvalidData)?,
            d.field(1)?.ok_or(DecodeError::InvalidData)?,
            d.field(2)?.ok_or(DecodeError::InvalidData)?,
            d.field(3)?.ok_or(DecodeError::InvalidData)?,
        ))
    }
}
