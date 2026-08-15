use kadent_engine::{
    data_types::AudioContext,
    timing::{TempoEvent, TempoMap},
};
use serde::{Deserialize, Serialize};

/// Mirror of `TempoMap` for persistence. `audio_ctx` is not stored here but it must
/// always match the project-wide `AudioContext`, so it's restored using `set_audio_ctx` after load.
#[derive(Serialize, Deserialize)]
pub(crate) struct StoredTempoMap {
    pub events: Vec<TempoEvent>,
}

impl StoredTempoMap {
    pub fn from_tempo_map(tempo_map: &TempoMap) -> Self {
        Self {
            events: tempo_map.events.clone(),
        }
    }

    pub fn to_tempo_map(&self, audio_ctx: &AudioContext) -> TempoMap {
        let mut tempo_map = TempoMap::new(audio_ctx.resolution, 120.0);
        tempo_map.events = self.events.clone();
        tempo_map
    }
}
