use crate::{
    core::audio_engine::{
        audio_data::AudioSource,
        track::{
            RegionID, Track,
            audio_track::{AudioRegion, AudioTrack},
        },
    },
    storage::project::serial::restore_next_id,
};
use sode::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder};
use std::{collections::HashMap, path::PathBuf};

// --- AudioTrack ---

impl Encode for AudioTrack {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, self.get_graph())?;
        e.field(1, self.get_all_regions())?;
        Ok(())
    }
}

impl Decode for AudioTrack {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        let graph = d.field(0)?.ok_or(DecodeError::InvalidData)?;
        let regions: HashMap<RegionID, AudioRegion> =
            d.field(1)?.ok_or(DecodeError::InvalidData)?;
        let next_id = restore_next_id(&regions.keys().copied().collect::<Vec<_>>());
        Ok(AudioTrack::with_initial(graph, regions, next_id))
    }
}

// --- AudioRegion ---

impl Encode for AudioRegion {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        let data_offset_u64: u64 = self
            .data_offset
            .try_into()
            .map_err(|_| EncodeError::InvalidLength)?;
        e.field(0, &self.data_source)?;
        e.field(1, &self.bounds)?;
        e.field(2, &data_offset_u64)?;
        e.field(3, &self.bpm)?;
        Ok(())
    }
}

impl Decode for AudioRegion {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        let data_source = d.field(0)?.ok_or(DecodeError::InvalidData)?;
        let bounds = d.field(1)?.ok_or(DecodeError::InvalidData)?;
        let data_offset: u64 = d.field(2)?.ok_or(DecodeError::InvalidData)?;
        let data_offset_usize = data_offset
            .try_into()
            .map_err(|_| DecodeError::InvalidLength)?;
        let bpm = d.field(3)?.ok_or(DecodeError::InvalidData)?;
        Ok(AudioRegion::new(
            data_source,
            bounds,
            data_offset_usize,
            bpm,
        ))
    }
}

// --- AudioSource ---

impl Encode for AudioSource {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        match self {
            AudioSource::Original(path) => {
                e.field(0, &0u32)?;
                e.field(1, &path.to_str().ok_or(EncodeError::InvalidData)?)?;
            }
            AudioSource::Modified(path) => {
                e.field(0, &1u32)?;
                e.field(1, &path.to_str().ok_or(EncodeError::InvalidData)?)?;
            }
            AudioSource::Zero => {
                e.field(0, &2u32)?;
            }
        }
        Ok(())
    }
}

impl Decode for AudioSource {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        let variant = d.field(0)?.ok_or(DecodeError::InvalidData)?;
        match variant {
            0 => {
                let path_str: String = d.field(1)?.ok_or(DecodeError::InvalidData)?;
                let path = PathBuf::from(path_str);
                Ok(AudioSource::Original(path))
            }
            1 => {
                let path_str: String = d.field(1)?.ok_or(DecodeError::InvalidData)?;
                let path = PathBuf::from(path_str);
                Ok(AudioSource::Modified(path))
            }
            2 => Ok(AudioSource::Zero),
            _ => Err(DecodeError::InvalidData),
        }
    }
}
