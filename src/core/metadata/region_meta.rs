use kadent_engine::data_types::Ticks;

#[derive(Debug, Clone)]
pub(crate) struct RegionMeta {
    pub name: String,
    pub start: Ticks,
    pub duration: Ticks,
    pub max_duration: Option<Ticks>,
}

impl RegionMeta {
    pub fn new(name: String, start: Ticks, duration: Ticks, max_duration: Option<Ticks>) -> Self {
        Self {
            name,
            start,
            duration,
            max_duration,
        }
    }

    // --- REGION MODIFICATION ---

    pub fn move_region(&mut self, new_start: Ticks) {
        self.start = new_start;
    }

    pub fn set_duration(&mut self, new_duration: Ticks) {
        self.duration = self
            .max_duration
            .map(|max| new_duration.min(max))
            .unwrap_or(new_duration);
    }
}
