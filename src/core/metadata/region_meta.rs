use kadent_engine::timing::TimeBounds;

#[derive(Debug, Clone)]
pub(crate) struct RegionMeta {
    pub name: String,
    pub bounds: TimeBounds,
}

impl RegionMeta {
    pub fn new(name: String, bounds: TimeBounds) -> Self {
        Self { name, bounds }
    }
}
