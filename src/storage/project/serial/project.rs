use super::restore_next_id;
use kadent_engine::{mixer::ProjectData, timing::TimeBounds};
use sode::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder};

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
        let tracks = d.field(0)?.unwrap_or_default();
        let tempo_map = d.field(1)?.unwrap_or_default();
        let audio_ctx = d.field(2)?.unwrap_or_default();
        let export_range = d.field(3)?.unwrap_or(TimeBounds::ZERO);

        let next_id = restore_next_id(tracks.keys());
        let mut data = ProjectData::with_all(tracks, tempo_map, audio_ctx, export_range, next_id);

        Ok(data)
    }
}
