use crate::core::audio_engine::timing::TimeBounds;

#[derive(Debug, Clone)]
pub(crate) struct RegionMeta {
    pub(crate) name: String,
    pub(crate) bounds: TimeBounds,
}

impl RegionMeta {
    pub(crate) fn new(name: String, bounds: TimeBounds) -> Self {
        Self { name, bounds }
    }
}
