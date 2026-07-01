use kadent_engine::data_types::Ticks;

#[derive(Default)]
pub(crate) enum Modification {
    #[default]
    None,
    ProjectRange(Ticks, Ticks),
    RegionRange(Ticks, Ticks),
    /// (start, end, pitch)
    NotePosition(Ticks, Ticks, f32),
}

impl Modification {
    pub(crate) fn is_none(&self) -> bool {
        matches!(self, Modification::None)
    }
}
