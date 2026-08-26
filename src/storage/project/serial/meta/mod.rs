use crate::core::{audio_engine::data_types::PlaybackContext, metadata::ProjectMeta};
use eframe::egui;
use sode::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder};

mod graph;
mod region;
mod track;

// --- ProjectMeta ---

impl Encode for ProjectMeta {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, &self.tracks)?;
        e.field(1, &self.track_order)?;
        e.field(2, &self.export_ctx)?;
        Ok(())
    }
}

impl Decode for ProjectMeta {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        Ok(ProjectMeta {
            tracks: d.field(0)?.ok_or(DecodeError::InvalidData)?,
            track_order: d.field(1)?.ok_or(DecodeError::InvalidData)?,
            export_ctx: d.field(2)?.ok_or(DecodeError::InvalidData)?,
        })
    }
}

// --- PlaybackContext ---

impl Encode for PlaybackContext {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        let channels_u64: u64 = self
            .channels
            .try_into()
            .map_err(|_| EncodeError::InvalidData)?;
        let buffer_size_u64: u64 = self
            .buffer_size
            .try_into()
            .map_err(|_| EncodeError::InvalidData)?;
        e.field(0, &channels_u64)?;
        e.field(1, &self.sample_rate)?;
        e.field(2, &buffer_size_u64)?;
        Ok(())
    }
}

impl Decode for PlaybackContext {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        let channels_u64: u64 = d.field(0)?.ok_or(DecodeError::InvalidData)?;
        let sample_rate = d.field(1)?.ok_or(DecodeError::InvalidData)?;
        let buffer_size_u64: u64 = d.field(2)?.ok_or(DecodeError::InvalidData)?;
        let channels = channels_u64
            .try_into()
            .map_err(|_| DecodeError::InvalidData)?;
        let buffer_size = buffer_size_u64
            .try_into()
            .map_err(|_| DecodeError::InvalidData)?;
        Ok(PlaybackContext {
            channels,
            sample_rate,
            buffer_size,
        })
    }
}

// --- StoredColor ---

struct StoredColor {
    r: u8,
    g: u8,
    b: u8,
}

impl StoredColor {
    fn to_color32(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.r, self.g, self.b)
    }

    fn from_color32(color: &egui::Color32) -> Self {
        StoredColor {
            r: color.r(),
            g: color.g(),
            b: color.b(),
        }
    }
}

impl Encode for StoredColor {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, &self.r)?;
        e.field(1, &self.g)?;
        e.field(2, &self.b)?;
        Ok(())
    }
}

impl Decode for StoredColor {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        Ok(StoredColor {
            r: d.field(0)?.ok_or(DecodeError::InvalidData)?,
            g: d.field(1)?.ok_or(DecodeError::InvalidData)?,
            b: d.field(2)?.ok_or(DecodeError::InvalidData)?,
        })
    }
}

// --- StoredPos2 ---

struct StoredPos2 {
    x: f32,
    y: f32,
}

impl StoredPos2 {
    fn to_pos2(&self) -> egui::Pos2 {
        egui::pos2(self.x, self.y)
    }

    fn from_pos2(pos: &egui::Pos2) -> Self {
        StoredPos2 { x: pos.x, y: pos.y }
    }
}

impl Encode for StoredPos2 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, &self.x)?;
        e.field(1, &self.y)?;
        Ok(())
    }
}

impl Decode for StoredPos2 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        Ok(StoredPos2 {
            x: d.field(0)?.ok_or(DecodeError::InvalidData)?,
            y: d.field(1)?.ok_or(DecodeError::InvalidData)?,
        })
    }
}
