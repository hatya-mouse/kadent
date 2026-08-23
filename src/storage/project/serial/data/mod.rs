use crate::core::audio_engine::{
    data_types::AudioContext,
    mixer::{ProjectData, TrackID},
    timing::TimeBounds,
    track::Track,
};
use sode::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder};
use std::collections::HashMap;

mod audio_track;
mod automation;
mod graph;
mod node;
mod note_track;
mod tempo_map;
mod track;

fn restore_next_id<T>(used_ids: &[T]) -> u64
where
    T: Into<u64> + Copy,
{
    used_ids
        .iter()
        .map(|id| Into::into(*id))
        .max()
        .map_or(0, |max_id| max_id + 1)
}

// --- ProjectData ---

impl Encode for ProjectData {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, &self.tracks)?;
        e.field(1, &self.tempo_map)?;
        e.field(2, &self.audio_ctx)?;
        e.field(3, &self.export_range)?;
        Ok(())
    }
}

impl Decode for ProjectData {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        let tracks: HashMap<TrackID, Box<dyn Track>> = d.field(0)?.unwrap_or_default();
        let tempo_map = d.field(1)?.unwrap_or_default();
        let audio_ctx = d.field(2)?.unwrap_or_default();
        let export_range = d.field(3)?.unwrap_or(TimeBounds::ZERO);
        let next_id = restore_next_id(&tracks.keys().copied().collect::<Vec<_>>());
        Ok(ProjectData::with_all(
            tracks,
            tempo_map,
            audio_ctx,
            export_range,
            next_id,
        ))
    }
}

// --- AudioContext ---

impl Encode for AudioContext {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, &self.resolution)?;
        Ok(())
    }
}

impl Decode for AudioContext {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        Ok(AudioContext {
            resolution: d.field(0)?.ok_or(DecodeError::InvalidData)?,
        })
    }
}
