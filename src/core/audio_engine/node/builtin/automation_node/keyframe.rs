use crate::core::audio_engine::{data_types::Ticks, node::builtin::CurveType};

#[derive(Debug, Clone)]
pub(crate) struct Keyframe<T> {
    pub(crate) tick: Ticks,
    pub(crate) curve: CurveType,
    pub(crate) value: T,
}

impl<T> Keyframe<T> {
    pub(crate) fn new(tick: Ticks, curve: CurveType, value: T) -> Self {
        Self { tick, curve, value }
    }
}

pub(crate) struct NormalizedKeyframe {
    pub(crate) index: usize,
    pub(crate) tick: Ticks,
    pub(crate) curve: CurveType,
    pub(crate) value: f32,
}

impl NormalizedKeyframe {
    pub(crate) fn new(index: usize, tick: Ticks, curve: CurveType, value: f32) -> Self {
        Self {
            index,
            tick,
            curve,
            value,
        }
    }
}
