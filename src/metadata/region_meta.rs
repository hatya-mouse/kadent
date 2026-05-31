use kreniq_engine::data_types::Beats;

#[derive(Debug)]
pub(crate) struct RegionMeta {
    pub name: String,
    pub start: Beats,
    pub duration: Beats,
    pub max_duration: Option<Beats>,
}

impl RegionMeta {
    pub fn new(name: String, start: Beats, duration: Beats, max_duration: Option<Beats>) -> Self {
        Self {
            name,
            start,
            duration,
            max_duration,
        }
    }

    // --- REGION MODIFICATION ---

    pub fn move_region(&mut self, new_start: Beats) {
        self.start = new_start;
    }

    pub fn set_duration(&mut self, new_duration: Beats) {
        self.duration = self
            .max_duration
            .map(|max| new_duration.min(max))
            .unwrap_or(new_duration);
    }
}
