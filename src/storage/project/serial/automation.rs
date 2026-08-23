use crate::core::audio_engine::node::builtin::{
    AutomationTrack, AutomationTrackType, CurveType, Keyframe,
};
use sode::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder};

// --- AutomationTrack ---

impl Encode for AutomationTrack {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, &self.track_type())?;
        match self {
            AutomationTrack::Float {
                keyframes, range, ..
            } => {
                e.field(1, keyframes)?;
                e.field(2, range.start())?;
                e.field(3, range.end())?;
            }
            AutomationTrack::Int {
                keyframes, range, ..
            } => {
                e.field(1, keyframes)?;
                e.field(2, range.start())?;
                e.field(3, range.end())?;
            }
            AutomationTrack::Bool { keyframes, .. } => {
                e.field(1, keyframes)?;
            }
        }
        Ok(())
    }
}

impl Decode for AutomationTrack {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        let variant = d.field(0)?.ok_or(DecodeError::InvalidData)?;
        match variant {
            0 => {
                let keyframes = d.field(1)?.ok_or(DecodeError::InvalidData)?;
                let range_start: f32 = d.field(2)?.ok_or(DecodeError::InvalidData)?;
                let range_end: f32 = d.field(3)?.ok_or(DecodeError::InvalidData)?;
                let range = range_start..=range_end;
                Ok(AutomationTrack::with_float_keyframes(keyframes, range))
            }
            1 => {
                let keyframes = d.field(1)?.ok_or(DecodeError::InvalidData)?;
                let range_start: i32 = d.field(2)?.ok_or(DecodeError::InvalidData)?;
                let range_end: i32 = d.field(3)?.ok_or(DecodeError::InvalidData)?;
                let range = range_start..=range_end;
                Ok(AutomationTrack::with_int_keyframes(keyframes, range))
            }
            2 => {
                let keyframes = d.field(1)?.ok_or(DecodeError::InvalidData)?;
                Ok(AutomationTrack::with_bool_keyframes(keyframes))
            }
            _ => Err(DecodeError::InvalidData),
        }
    }
}

// --- AutomationTrackType ---

impl Encode for AutomationTrackType {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_u32(*self as u32);
        Ok(())
    }
}

impl Decode for AutomationTrackType {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        match d.read_u32()? {
            0 => Ok(AutomationTrackType::Float),
            1 => Ok(AutomationTrackType::Int),
            2 => Ok(AutomationTrackType::Bool),
            _ => Err(DecodeError::InvalidData),
        }
    }
}

// --- Keyframe<T> ---

impl<T: Encode> Encode for Keyframe<T> {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, &self.tick)?;
        e.field(1, &self.curve)?;
        e.field(2, &self.value)?;
        Ok(())
    }
}

impl<T: Decode> Decode for Keyframe<T> {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        Ok(Keyframe {
            tick: d.field(0)?.ok_or(DecodeError::InvalidData)?,
            curve: d.field(1)?.ok_or(DecodeError::InvalidData)?,
            value: d.field(2)?.ok_or(DecodeError::InvalidData)?,
        })
    }
}

// --- CurveType ---

impl Encode for CurveType {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        match self {
            CurveType::Linear => {
                e.field(0, &0u32)?;
            }
            CurveType::Step => {
                e.field(0, &1u32)?;
            }
            CurveType::Smooth { tension } => {
                e.field(0, &2u32)?;
                e.field(1, tension)?;
            }
        }
        Ok(())
    }
}

impl Decode for CurveType {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        let variant = d.field(0)?.ok_or(DecodeError::InvalidData)?;
        match variant {
            0 => Ok(CurveType::Linear),
            1 => Ok(CurveType::Step),
            2 => {
                let tension: f32 = d.field(1)?.ok_or(DecodeError::InvalidData)?;
                Ok(CurveType::Smooth { tension })
            }
            _ => Err(DecodeError::InvalidData),
        }
    }
}
