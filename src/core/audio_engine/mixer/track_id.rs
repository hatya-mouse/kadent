#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Debug)]
pub(crate) struct TrackID(pub(crate) u64);

impl Into<u64> for TrackID {
    fn into(self) -> u64 {
        self.0
    }
}

impl From<u64> for TrackID {
    fn from(value: u64) -> Self {
        TrackID(value)
    }
}
