use crate::core::audio_engine::{
    data_types::Ticks,
    graph::node_id::NodeID,
    mixer::TrackID,
    timing::TimeBounds,
    track::{RegionID, note_track::NoteID},
};
use sode::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder};

// --- TrackID ---

impl Encode for TrackID {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_u64(self.0);
        Ok(())
    }
}

impl Decode for TrackID {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        Ok(TrackID(d.read_u64()?))
    }
}

// --- RegionID ---

impl Encode for RegionID {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_u64(self.0);
        Ok(())
    }
}

impl Decode for RegionID {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        Ok(RegionID(d.read_u64()?))
    }
}

// --- NodeID ---

impl Encode for NodeID {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_u64(self.0);
        Ok(())
    }
}

impl Decode for NodeID {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        Ok(NodeID(d.read_u64()?))
    }
}

// --- NoteID ---

impl Encode for NoteID {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_u64(self.0);
        Ok(())
    }
}

impl Decode for NoteID {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        Ok(NoteID(d.read_u64()?))
    }
}

// --- Ticks ---

impl Encode for Ticks {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_i64(self.0);
        Ok(())
    }
}

impl Decode for Ticks {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        Ok(Ticks(d.read_i64()?))
    }
}

// --- TimeBounds ---

impl Encode for TimeBounds {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        match self {
            TimeBounds::Musical { start, duration } => {
                e.field(0, &0u32)?;
                e.field(1, start)?;
                e.field(2, duration)?;
            }
            TimeBounds::Time {
                start_seconds,
                duration_seconds,
            } => {
                e.field(0, &1u32)?;
                e.field(1, start_seconds)?;
                e.field(2, duration_seconds)?;
            }
        }
        Ok(())
    }
}

impl Decode for TimeBounds {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        let variant: u32 = d.field(0)?.ok_or(DecodeError::InvalidData)?;
        match variant {
            0 => Ok(TimeBounds::Musical {
                start: d.field(1)?.ok_or(DecodeError::InvalidData)?,
                duration: d.field(2)?.ok_or(DecodeError::InvalidData)?,
            }),
            1 => Ok(TimeBounds::Time {
                start_seconds: d.field(1)?.ok_or(DecodeError::InvalidData)?,
                duration_seconds: d.field(2)?.ok_or(DecodeError::InvalidData)?,
            }),
            _ => Err(DecodeError::InvalidData),
        }
    }
}
